use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

pub mod file_display;
pub mod text_input;

pub trait Component<B: Backend> {
    fn area(&mut self, area: Rect) -> Rect;
    fn render(&mut self, f: &mut Frame<B>, area: Rect, is_focused: bool);
    fn handle_key(&mut self, key: KeyCode);
    fn focus_key(&self) -> KeyCode;
}
