use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::component::Component;

pub struct App<B: Backend> {
    pub do_search: bool,
    components: Option<[Box<dyn Component<B>>; 0]>,
    pub focused_index: Option<usize>,
    min_width: u16,
    min_height: u16,
}

#[allow(clippy::new_without_default)]
impl<B: Backend> App<B> {
    pub fn new() -> Self {
        let mut components: [Box<dyn Component<B>>; 0] = [];

        let mut min_width = 0;
        let mut min_height = 0;

        for component in components.iter_mut() {
            let mut rect = Rect::new(0, 0, 0, 0);
            rect = component.area(rect);
            let width = rect.x + rect.width;

            if min_width < width {
                min_width = width;
            }

            let height = rect.y + rect.height;

            if min_height < height {
                min_height = height;
            }
        }

        Self {
            do_search: false,
            components: Some(components),
            focused_index: None,
            min_width,
            min_height,
        }
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

            let area = component.area(f.size());

            component.render(f, area, is_focused);
        }

        self.components = Some(components);
    }
}
