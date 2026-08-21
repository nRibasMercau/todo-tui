use crate::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn update_normal(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') => app.select_next(),
        KeyCode::Char('k') => app.select_previous(),
        KeyCode::Char(' ') => app.toggle_status(),
        KeyCode::Enter => app.open_todo_popup(),
        _ => {}
    };
}

fn update_edit(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.popup = None;
            return;
        }
        _ => {}
    }

    if let Some(popup) = app.popup.as_mut() {
        match key_event.code {
            KeyCode::Tab => {
                if key_event.modifiers == KeyModifiers::SHIFT {
                    popup.focus_previous();
                } else {
                    popup.focus_next();
                }
            }
            _ => {}
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    if app.popup.is_some() {
        update_edit(app, key_event);
    } else {
        update_normal(app, key_event);
    }
}
