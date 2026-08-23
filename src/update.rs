use crate::app::{App, Focus};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/*
 * List mode
 * The following key bindings will apply only when there is no popup
 * open and the list is visible
 */
fn update_normal(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') => app.select_next(),
        KeyCode::Char('k') => app.select_previous(),
        KeyCode::Char(' ') => app.todo_list.toggle_status(),
        KeyCode::Enter => app.open_todo_popup(app.todo_list.state.selected()),
        KeyCode::Char('a') => app.open_todo_popup(None),
        _ => {}
    };
}

/*
 * Edit mode
 * The following key bindings will apply only when there is a popup
 * open and we are in edit mode
 */
fn update_edit(app: &mut App, key_event: KeyEvent) {
    // Esc to leave edit mode ang go back to the list
    // We can't use q, because that's a valid character
    match key_event.code {
        KeyCode::Esc => {
            app.popup = None;
            return;
        }
        _ => {}
    }

    if let Some(popup) = app.popup.as_mut() {
        match key_event.code {
            // Tab changes focus
            KeyCode::Tab => {
                if key_event.modifiers == KeyModifiers::SHIFT {
                    popup.focus_previous();
                } else {
                    popup.focus_next();
                }
            }

            // Spacebar adds a space in StringField, does nothing status in Status
            KeyCode::Char(' ') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::Tag => popup.tag.on_key_press(key_event),
            },

            // Arrows toggle status in Status
            KeyCode::Left => match popup.focus {
                Focus::Todo => popup.todo.cursor_left(),
                Focus::Info => popup.info.cursor_left(),
                Focus::Status => popup.status = popup.status.previous(),
                Focus::Tag => popup.tag.cursor_left(),
            },
            KeyCode::Right => match popup.focus {
                Focus::Todo => popup.todo.cursor_right(),
                Focus::Info => popup.info.cursor_right(),
                Focus::Status => popup.status = popup.status.next(),
                Focus::Tag => popup.tag.cursor_right(),
            },

            // Enter submits the form
            KeyCode::Enter => {
                let new_todo_item = popup.submit();
                if let Some(index) = popup.editing {
                    match app.todo_list.replace_todo(new_todo_item, index) {
                        Ok(()) => app.popup = None,
                        Err(error) => {
                            // Save failed
                            app.error_message = Some(format!("Error saving todo: {error:?}"))
                        }
                    }
                } else {
                    app.todo_list.add_todo(new_todo_item);
                    app.popup = None
                }
            }

            // Other characters insert characters in StringField, does nothing in Status
            _ => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::Tag => popup.tag.on_key_press(key_event),
            },
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    if app.popup.is_some() {
        // Edit mode
        update_edit(app, key_event);
    } else {
        // List mode
        update_normal(app, key_event);
    }
}
