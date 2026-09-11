use crate::app::ActivePanel;
use crate::app::App;
use crate::ui::calendar;
use crate::ui::todo_popup::Focus;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/*
 * List mode
 * The following key bindings will apply only when there is no popup
 * open and the list is visible
 */
fn update_normal(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('h') => app.active_panel = ActivePanel::Projects,
        KeyCode::Char('l') => app.active_panel = ActivePanel::Todos,
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),

        code => match app.active_panel {
            ActivePanel::Todos => match code {
                KeyCode::Char('j') => app.todo_list.select_next(),
                KeyCode::Char('k') => app.todo_list.select_previous(),
                KeyCode::Char(' ') => app.toggle_status_todo().unwrap(),
                KeyCode::Enter => app.open_todo_popup(app.todo_list.state.selected()),
                KeyCode::Char('a') => app.open_todo_popup(None),
                _ => {}
            },
            ActivePanel::Projects => match code {
                KeyCode::Char('j') => app.projects.select_next(),
                KeyCode::Char('k') => app.projects.select_previous(),
                _ => {}
            },
        },
    }
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

        // Enter submits the form
        KeyCode::Enter => {
            if app.popup.as_ref().unwrap().focus == Focus::DueDate {
                let popup = app.popup.as_mut().unwrap();
                popup.due_date = Some(popup.calendar_date);
                popup.focus_next();
            } else {
                match app.submit_todo() {
                    Ok(()) => {
                        app.popup = None;
                        app.error_message = None;
                    }
                    Err(err) => {
                        app.error_message = Some(err.to_string());
                    }
                }
            }
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

            KeyCode::Char(' ') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => {}
                Focus::Project => popup.project.on_key_press(key_event),
            },

            // Arrows toggle status in Status
            KeyCode::Left => match popup.focus {
                Focus::Todo => popup.todo.cursor_left(),
                Focus::Info => popup.info.cursor_left(),
                Focus::Status => popup.status = popup.status.previous(),
                Focus::DueDate => {}
                Focus::Project => popup.project.cursor_left(),
            },
            KeyCode::Right => match popup.focus {
                Focus::Todo => popup.todo.cursor_right(),
                Focus::Info => popup.info.cursor_right(),
                Focus::Status => popup.status = popup.status.next(),
                Focus::DueDate => {}
                Focus::Project => popup.project.cursor_right(),
            },

            KeyCode::Char('p') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::prev_month(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            KeyCode::Char('n') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::next_month(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            KeyCode::Char('j') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::move_down(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            KeyCode::Char('k') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::move_up(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            KeyCode::Char('h') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::move_left(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            KeyCode::Char('l') => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => popup.calendar_date = calendar::move_right(popup.calendar_date),
                Focus::Project => popup.project.on_key_press(key_event),
            },

            // Other characters insert characters in StringField, does nothing in Status
            _ => match popup.focus {
                Focus::Todo => popup.todo.on_key_press(key_event),
                Focus::Info => popup.info.on_key_press(key_event),
                Focus::Status => {}
                Focus::DueDate => {}
                Focus::Project => popup.project.on_key_press(key_event),
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
