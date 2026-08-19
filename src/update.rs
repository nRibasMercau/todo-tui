use crate::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') => app.select_next(),
        KeyCode::Char('k') => app.select_previous(),
        _ => {}
    };
}
