use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::component::Component;

pub struct App<B: Backend> {
    components: [Box<dyn Component<B>>; 0],
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
            components,
            focused_index: None,
            min_width,
            min_height,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        if self.focused_index.is_none() {
            for (i, component) in self.components.iter().enumerate() {
                if component.focus_key() == key {
                    self.focused_index = Some(i);
                    break;
                }
            }
            return;
        }

        let focused_component = self
            .components
            .get_mut(self.focused_index.unwrap())
            .unwrap();

        focused_component.handle_key(key);

        if key == KeyCode::Esc {
            self.focused_index = None;
        }
    }

    pub fn draw(&mut self, f: &mut Frame<B>) {
        let frame_size = f.size();

        if frame_size.width < self.min_width || frame_size.height < self.min_height {
            return;
        }

        for (i, component) in self.components.iter_mut().enumerate() {
            let is_focused = self.focused_index.is_some() && self.focused_index.unwrap() == i;

            let area = component.area(f.size());

            component.render(f, area, is_focused);
        }
    }
}
