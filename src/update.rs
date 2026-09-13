use crate::app::ActivePanel;
use crate::app::{App, Dialog};
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
        KeyCode::Char('h') => {
            if app.active_panel == ActivePanel::Todos {
                app.active_panel = ActivePanel::Projects
            }
        }
        KeyCode::Char('l') => {
            if app.active_panel == ActivePanel::Projects {
                app.active_panel = ActivePanel::Todos
            }
        }
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
                KeyCode::Enter => app.open_project_popup(app.projects.state.selected()),
                KeyCode::Char('a') => app.open_project_popup(None),
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
            app.dialog = None;
            return;
        }

        // Enter submits the form
        KeyCode::Enter => {
            if let Some(Dialog::Todo(popup)) = app.dialog.as_mut() {
                if popup.focus == Focus::DueDate {
                    popup.due_date = Some(popup.calendar_date);
                    popup.focus_next();
                } else {
                    match app.submit_todo() {
                        Ok(()) => {
                            app.dialog = None;
                            app.error_message = None;
                        }
                        Err(err) => {
                            app.error_message = Some(err.to_string());
                        }
                    }
                }
            }

            if let Some(Dialog::Project(_)) = app.dialog.as_mut() {
                match app.submit_project() {
                    Ok(()) => {
                        app.dialog = None;
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

    if let Some(Dialog::Todo(popup)) = app.dialog.as_mut() {
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
    if let Some(Dialog::Project(popup)) = app.dialog.as_mut() {
        match key_event.code {
            _ => popup.name.on_key_press(key_event),
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    if let Some(dialog) = &app.dialog {
        match dialog {
            Dialog::Todo(_) => update_edit(app, key_event),
            Dialog::Project(_) => update_edit(app, key_event),
            _ => {}
        }
    } else {
        update_normal(app, key_event);
    }
}
