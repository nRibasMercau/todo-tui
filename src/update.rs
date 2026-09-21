use crate::app::{ActivePanel, App, Dialog};
use crate::ui::calendar;
use crate::ui::confirm_popup::{ConfirmAction, ConfirmPopup};
use crate::ui::project_popup::{ProjectPopup, ProjectPopupMode};
use crate::ui::todo_popup::{Focus, TodoPopup, TodoPopupMode};
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
                KeyCode::Char('j') => app.todo_table.select_next(),
                KeyCode::Char('k') => app.todo_table.select_previous(),
                KeyCode::Char(' ') => {
                    match app.toggle_status_todo(app.todo_table.state.selected()) {
                        Ok(()) => {}
                        Err(err) => app.error_message = Some(err.to_string()),
                    }
                }
                KeyCode::Enter => {
                    if let Some(index) = app.todo_table.state.selected() {
                        app.edit_todo(index);
                    }
                }
                KeyCode::Char('a') => app.add_todo(),
                KeyCode::Char('d') => {
                    if let Some(index) = app.todo_table.state.selected() {
                        app.confirm_delete_todo(index)
                    };
                }
                _ => {}
            },
            ActivePanel::Projects => match code {
                KeyCode::Char('j') => app.projects.select_next(),
                KeyCode::Char('k') => app.projects.select_previous(),
                KeyCode::Enter => {
                    if let Some(index) = app.projects.state.selected() {
                        app.edit_project(index);
                    }
                }
                KeyCode::Char('a') => app.add_project(),
                KeyCode::Char('d') => {
                    if let Some(index) = app.projects.state.selected() {
                        app.confirm_delete_project(index)
                    };
                }
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
    if key_event.code == KeyCode::Esc {
        app.close_dialog();
        return;
    }

    let Some(dialog) = app.dialog.take() else {
        return;
    };

    app.dialog = match dialog {
        Dialog::Todo(popup) => update_todo(app, popup, key_event).map(Dialog::Todo),
        Dialog::Project(popup) => update_project(app, popup, key_event).map(Dialog::Project),
        Dialog::Confirm(popup) => update_confirm(app, popup, key_event).map(Dialog::Confirm),
    };

    /*
    match dialog {
        Dialog::Todo(popup) => match popup.mode {
            TodoPopupMode::Create => app.create_todo(popup.into_new_todo()),
            TodoPopupMode::Edit(todo_id) => app.update_todo(todo_id),
        },
        Dialog::Project(popup) => {}
        Dialog::Confirm(popup) => {}
    }
    let action = match app.dialog.as_mut().unwrap() {
        Dialog::Todo(popup) => update_todo(popup, key_event),
        Dialog::Project(popup) => update_project(popup, key_event),
        Dialog::Confirm(popup) => update_confirm(popup, key_event),
        Dialog::None => DialogAction::None,
    };

    match action {
        DialogAction::SubmitTodo => {
            if let Some(index) = app.todo_table.state.selected() {
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
        DialogAction::SubmitProject => match app.submit_project() {
            Ok(()) => {
                app.dialog = None;
                app.error_message = None;
            }
            Err(err) => {
                app.error_message = Some(err.to_string());
            }
        },
        DialogAction::Confirm(confirm_action) => match confirm_action {
            ConfirmAction::DeleteProject(project_id) => match app.delete_project(project_id) {
                Ok(()) => {
                    app.dialog = None;
                    app.error_message = None;
                }
                Err(err) => {
                    app.error_message = Some(err.to_string());
                }
            },
            ConfirmAction::DeleteTodo(todo_id) => match app.delete_todo(todo_id) {
                Ok(()) => {
                    app.dialog = None;
                    app.error_message = None;
                }
                Err(err) => {
                    app.error_message = Some(err.to_string());
                }
            },
        },
        DialogAction::Close => {
            app.dialog = None;
        }
        DialogAction::None => {}
    }
    */
}

fn update_confirm(app: &mut App, popup: ConfirmPopup, key_event: KeyEvent) -> Option<ConfirmPopup> {
    match key_event.code {
        KeyCode::Char('y') => match popup.action {
            ConfirmAction::DeleteTodo(todo_id) => match app.delete_todo(todo_id) {
                Ok(()) => {
                    app.error_message = None;
                    return None;
                }
                Err(err) => app.error_message = Some(err.to_string()),
            },
            ConfirmAction::DeleteProject(project_id) => match app.delete_project(project_id) {
                Ok(()) => {
                    app.error_message = None;
                    return None;
                }
                Err(err) => app.error_message = Some(err.to_string()),
            },
        },
        KeyCode::Char('n') | KeyCode::Esc => app.close_dialog(),
        _ => {}
    }
    Some(popup)
}

fn update_todo(app: &mut App, mut popup: TodoPopup, key_event: KeyEvent) -> Option<TodoPopup> {
    match key_event.code {
        // Tab changes focus
        KeyCode::Tab => {
            if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                popup.focus_previous();
            } else {
                popup.focus_next();
            }
        }
        KeyCode::Enter => match popup.focus {
            Focus::DueDate => {
                popup.due_date = Some(popup.calendar_date);
                popup.focus_next();
            }
            _ => {
                match popup.mode {
                    TodoPopupMode::Create => match app.create_todo(popup.into_new_todo()) {
                        Ok(()) => {
                            app.error_message = None;
                            return None;
                        }
                        Err(err) => app.error_message = Some(err.to_string()),
                    },
                    TodoPopupMode::Edit(todo_id) => {
                        match app.update_todo(todo_id, popup.into_form_data()) {
                            Ok(()) => {
                                app.error_message = None;
                                return None;
                            }
                            Err(err) => app.error_message = Some(err.to_string()),
                        }
                    }
                };
            }
        },
        other_key => match popup.focus {
            Focus::Todo => match other_key {
                KeyCode::Left => popup.todo.cursor_left(),
                KeyCode::Right => popup.todo.cursor_right(),
                _ => popup.todo.on_key_press(key_event),
            },
            Focus::Info => match other_key {
                KeyCode::Left => popup.info.cursor_left(),
                KeyCode::Right => popup.info.cursor_right(),
                _ => popup.info.on_key_press(key_event),
            },
            Focus::Status => match other_key {
                KeyCode::Char('j') => popup.status = popup.status.next(),
                KeyCode::Char('k') => popup.status = popup.status.previous(),
                _ => {}
            },
            Focus::DueDate => match other_key {
                KeyCode::Char('p') => {
                    popup.calendar_date = calendar::prev_month(popup.calendar_date)
                }
                KeyCode::Char('n') => {
                    popup.calendar_date = calendar::next_month(popup.calendar_date)
                }
                KeyCode::Char('j') => {
                    popup.calendar_date = calendar::move_down(popup.calendar_date)
                }
                KeyCode::Char('k') => popup.calendar_date = calendar::move_up(popup.calendar_date),
                KeyCode::Char('h') => {
                    popup.calendar_date = calendar::move_left(popup.calendar_date)
                }
                KeyCode::Char('l') => {
                    popup.calendar_date = calendar::move_right(popup.calendar_date)
                }
                _ => {}
            },
            Focus::Project => match other_key {
                KeyCode::Left => popup.project.cursor_left(),
                KeyCode::Right => popup.project.cursor_right(),
                _ => popup.project.on_key_press(key_event),
            },
        },
    }
    Some(popup)
}

fn update_project(
    app: &mut App,
    mut popup: ProjectPopup,
    key_event: KeyEvent,
) -> Option<ProjectPopup> {
    // Enter submits the form
    match key_event.code {
        KeyCode::Esc => app.close_dialog(),
        KeyCode::Enter => {
            match popup.mode {
                ProjectPopupMode::Create => match app.create_project(popup.into_new_project()) {
                    Ok(()) => {
                        app.error_message = None;
                        return None;
                    }
                    Err(err) => app.error_message = Some(err.to_string()),
                },
                ProjectPopupMode::Edit(project_id) => {
                    match app.update_project(project_id, popup.into_form_data()) {
                        Ok(()) => {
                            app.error_message = None;
                            return None;
                        }
                        Err(err) => app.error_message = Some(err.to_string()),
                    }
                }
            };
        }
        KeyCode::Left => {
            popup.name.cursor_left();
        }
        KeyCode::Right => {
            popup.name.cursor_right();
        }
        _ => {
            popup.name.on_key_press(key_event);
        }
    }
    Some(popup)
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    if app.dialog.is_some() {
        update_edit(app, key_event);
    } else {
        update_normal(app, key_event);
    }
}
