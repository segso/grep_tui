use std::any::Any;

use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::app::App;

pub mod file_display;
pub mod text_input;

pub trait Component<B: Backend> {
    fn as_any(&mut self) -> &mut dyn Any;
    fn render(&mut self, f: &mut Frame<B>, area: Rect, is_focused: bool);
    fn handle_key(&mut self, key: KeyCode, app: &mut App<B>);
    fn focus_key(&self) -> KeyCode;
}
