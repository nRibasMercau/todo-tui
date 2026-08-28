use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Offset, Position, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Padding},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct StringField {
    pub label: &'static str,
    pub value: String,
    pub cursor: usize,
}

impl StringField {
    pub fn new(label: &'static str, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            label,
            cursor: value.len(),
            value,
        }
    }

    pub fn blank(label: &'static str) -> Self {
        Self {
            label,
            value: String::new(),
            cursor: 0,
        }
    }

    pub fn stringfield_to_string(self) -> String {
        self.value
    }

    /// Handle input events for string input
    pub fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            _ => {}
        }
    }

    pub fn cursor_offset(&self) -> Offset {
        let x = (self.label.len() + self.value.len()) as i32;
        Offset::new(x, 0)
    }

    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    pub fn cursor_position(&self, area: Rect) -> Position {
        #[allow(unused_variables)]
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ]));

        let value_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default())
            .padding(Padding::horizontal(1));

        let value_inner = value_block.inner(value_area);

        Position {
            x: value_inner.x + self.cursor as u16,
            y: value_inner.y,
        }
    }
}

impl fmt::Display for StringField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
