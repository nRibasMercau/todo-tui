use crate::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') => app.select_next(),
        KeyCode::Char('k') => app.select_previous(),
        KeyCode::Char(' ') => app.toggle_status(),
        _ => {}
    };
}
