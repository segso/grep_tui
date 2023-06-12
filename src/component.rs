use crossterm::event::KeyCode;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::app::App;

pub mod file_display;
pub mod text_input;

pub trait Component<B: Backend> {
    fn text(&mut self) -> String;
    fn set_items(&mut self, items: Vec<(u32, String)>);
    fn render(&mut self, f: &mut Frame<B>, area: Rect, is_focused: bool);
    fn handle_key(&mut self, key: KeyCode, app: &mut App<B>);
    fn focus_key(&self) -> KeyCode;
}
