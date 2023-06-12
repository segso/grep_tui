use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::{
    component::{file_display::FileDisplay, text_input::TextInput, Component},
    grep::{grep_from_file, GrepError},
};

pub struct App<B: Backend> {
    pub do_search: bool,
    components: Option<[Box<dyn Component<B>>; 3]>,
    pub focused_index: Option<usize>,
    min_width: u16,
    min_height: u16,
}

#[allow(clippy::new_without_default)]
impl<B: Backend> App<B> {
    pub fn new() -> Self {
        let mut components: [Box<dyn Component<B>>; 3] = [
            Box::new(TextInput::new(
                String::from("Search"),
                String::from("Foo"),
                KeyCode::Char('/'),
            )),
            Box::new(TextInput::new(
                String::from("File"),
                String::from("./folder/file.txt"),
                KeyCode::Char('f'),
            )),
            Box::new(FileDisplay::new(
                String::from("Results"),
                KeyCode::Char('r'),
            )),
        ];

        let mut app = Self {
            do_search: false,
            components: None,
            focused_index: None,
            min_width: 0,
            min_height: 0,
        };

        let mut min_width = 0;
        let mut min_height = 0;

        for component in components.iter_mut() {
            let mut rect = Rect::new(0, 0, 0, 0);
            rect = app.area(rect, component.focus_key());
            let width = rect.x + rect.width;

            if min_width < width {
                min_width = width;
            }

            let height = rect.y + rect.height;

            if min_height < height {
                min_height = height;
            }
        }

        app.min_width = min_width;
        app.min_height = min_height;
        app.components = Some(components);

        app
    }

    pub fn area(&mut self, mut rect: Rect, focus_key: KeyCode) -> Rect {
        match focus_key {
            KeyCode::Char('/') => {
                rect.height = 3;
            }
            KeyCode::Char('f') => {
                rect.y = 3;
                rect.height = 3;
            }
            KeyCode::Char('r') => {
                if rect.height >= 6 {
                    rect.height -= 6;
                }
                rect.y = 6;
            }
            _ => {}
        }
        rect
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        let mut components = self.components.take().unwrap();

        if self.focused_index.is_none() {
            for (i, component) in components.iter().enumerate() {
                if component.focus_key() == key {
                    self.focused_index = Some(i);
                    break;
                }
            }
            self.components = Some(components);
            return;
        }

        let focused_component = components.get_mut(self.focused_index.unwrap()).unwrap();

        focused_component.handle_key(key, self);

        if key == KeyCode::Esc {
            self.focused_index = None;
        }

        self.components = Some(components);
    }

    pub fn search(&mut self) {
        let mut components = self.components.take().unwrap();

        let search_text = components[0]
            .as_any()
            .downcast_mut::<TextInput>()
            .unwrap()
            .text();

        let file_text = components[1]
            .as_any()
            .downcast_mut::<TextInput>()
            .unwrap()
            .text();

        let results = match grep_from_file(file_text, search_text) {
            Ok(results) => results,
            Err(err) => {
                self.components = Some(components);

                let (GrepError::FileNotFound | GrepError::PathIsNotFile) = err else {
                    return;
                };

                self.components.as_mut().unwrap()[1]
                    .as_any()
                    .downcast_mut::<TextInput>()
                    .unwrap()
                    .error();
                return;
            }
        };

        if results.is_empty() {
            components[0]
                .as_any()
                .downcast_mut::<TextInput>()
                .unwrap()
                .error();
        }

        let file_display: &mut FileDisplay = components[2].as_any().downcast_mut().unwrap();
        file_display.set_items(results);

        self.components = Some(components);
        self.do_search = false;
    }

    pub fn draw(&mut self, f: &mut Frame<B>) {
        if self.do_search {
            self.search();
        }

        let frame_size = f.size();

        if frame_size.width < self.min_width || frame_size.height < self.min_height {
            return;
        }

        let mut components = self.components.take().unwrap();

        for (i, component) in components.iter_mut().enumerate() {
            let is_focused = self.focused_index.is_some() && self.focused_index.unwrap() == i;

            let area = self.area(f.size(), component.focus_key());

            component.render(f, area, is_focused);
        }

        self.components = Some(components);
    }
}
