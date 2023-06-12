use std::any::Any;

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

use super::Component;

pub struct TextInput {
    title: String,
    placeholder: String,
    focus_key: KeyCode,
    error: bool,
    text: String,
}

impl TextInput {
    pub fn new(title: String, placeholder: String, focus_key: KeyCode) -> Self {
        Self {
            title,
            placeholder,
            focus_key,
            error: false,
            text: String::new(),
        }
    }

    pub fn text(&mut self) -> String {
        self.text.clone()
    }

    pub fn error(&mut self, error: bool) {
        self.error = error;
    }
}

impl<B: Backend> Component<B> for TextInput {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn render(&mut self, f: &mut Frame<B>, area: Rect, is_focused: bool) {
        let KeyCode::Char(focus_key) = self.focus_key else {
            return;
        };

        let block = Block::default()
            .title(Spans::from(vec![
                Span::styled(
                    String::from(" ") + &self.title,
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    format!(" [{focus_key}] "),
                    Style::default().fg(Color::Yellow),
                ),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.error {
                Color::Red
            } else if is_focused {
                Color::Blue
            } else {
                Color::White
            }));

        let placeholder = self.text.is_empty() && !is_focused;

        let text = if placeholder {
            &self.placeholder
        } else {
            &self.text
        };

        let paragraph = Paragraph::new(Spans::from(vec![
            Span::styled(
                text,
                Style::default().fg(if placeholder {
                    Color::DarkGray
                } else {
                    Color::White
                }),
            ),
            if is_focused {
                Span::styled(
                    "⎸",
                    Style::default().fg(if self.error { Color::Red } else { Color::Blue }),
                )
            } else {
                Span::raw("")
            },
        ]))
        .block(block);

        f.render_widget(paragraph, area);
    }

    fn focus_key(&self) -> KeyCode {
        self.focus_key
    }

    fn handle_key(&mut self, key: KeyCode, app: &mut App<B>) {
        if key == KeyCode::Backspace {
            app.do_search = true;
            self.text.pop();
            self.error = false;
            return;
        }

        if let KeyCode::Char(c) = key {
            app.do_search = true;
            self.text.push(c);
            self.error = false;
        }
    }
}
