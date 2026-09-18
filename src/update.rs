use crate::app::{ActivePanel, App, Dialog};
use crate::ui::calendar;
use crate::ui::confirm_popup::{ConfirmAction, ConfirmPopup};
use crate::ui::project_popup::ProjectPopup;
use crate::ui::todo_popup::{Focus, TodoPopup};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

enum DialogAction {
    None,
    Close,
    SubmitTodo,
    SubmitProject,
    Confirm(ConfirmAction),
}

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
                KeyCode::Char(' ') => {
                    let todo = &app.todo_list.items[app.todo_list.state.selected().unwrap()];
                    match app.toggle_status_todo(todo.id) {
                        Ok(()) => {}
                        Err(err) => app.error_message = Some(err.to_string()),
                    }
                }
                KeyCode::Enter => app.open_todo_popup(app.todo_list.state.selected()),
                KeyCode::Char('a') => app.open_todo_popup(None),
                KeyCode::Char('d') => {
                    let todo = &app.todo_list.items[app.todo_list.state.selected().unwrap()];
                    app.open_confirm_popup(
                        String::from("Confirm"),
                        format!("Delete task {:?}?", todo.todo),
                        ConfirmAction::DeleteTodo(todo.id),
                    )
                }
                _ => {}
            },
            ActivePanel::Projects => match code {
                KeyCode::Char('j') => app.projects.select_next(),
                KeyCode::Char('k') => app.projects.select_previous(),
                KeyCode::Enter => app.open_project_popup(app.projects.state.selected()),
                KeyCode::Char('a') => app.open_project_popup(None),
                KeyCode::Char('d') => {
                    let project = &app.projects.items[app.projects.state.selected().unwrap()];
                    app.open_confirm_popup(
                        String::from("Confirm"),
                        format!("Delete project {:?}?", project.name),
                        ConfirmAction::DeleteProject(project.id),
                    )
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
        app.dialog = None;
        return;
    }

    let action = match app.dialog.as_mut().unwrap() {
        Dialog::Todo(popup) => update_todo(popup, key_event),
        Dialog::Project(popup) => update_project(popup, key_event),
        Dialog::Confirm(popup) => update_confirm(popup, key_event),
        Dialog::None => DialogAction::None,
    };

    match action {
        DialogAction::SubmitTodo => match app.submit_todo() {
            Ok(()) => {
                app.dialog = None;
                app.error_message = None;
            }
            Err(err) => {
                app.error_message = Some(err.to_string());
            }
        },
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
}

fn update_confirm(popup: &ConfirmPopup, key_event: KeyEvent) -> DialogAction {
    match key_event.code {
        KeyCode::Char('y') => DialogAction::Confirm(popup.action.clone()),
        KeyCode::Char('n') | KeyCode::Esc => DialogAction::Close,
        _ => DialogAction::None,
    }
}

fn update_todo_enter(popup: &mut TodoPopup) -> DialogAction {
    match popup.focus {
        Focus::DueDate => {
            popup.due_date = Some(popup.calendar_date);
            popup.focus_next();
            DialogAction::None
        }
        _ => DialogAction::SubmitTodo,
    }
}

fn update_todo_space(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => {}
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_left(popup: &mut TodoPopup) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.cursor_left(),
        Focus::Info => popup.info.cursor_left(),
        Focus::Status => popup.status = popup.status.previous(),
        Focus::DueDate => {}
        Focus::Project => popup.project.cursor_left(),
    }
    DialogAction::None
}

fn update_todo_right(popup: &mut TodoPopup) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.cursor_right(),
        Focus::Info => popup.info.cursor_right(),
        Focus::Status => popup.status = popup.status.next(),
        Focus::DueDate => {}
        Focus::Project => popup.project.cursor_right(),
    }
    DialogAction::None
}

fn update_todo_p(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::prev_month(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_n(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::next_month(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_j(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::move_down(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_k(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::move_up(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_h(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::move_left(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_l(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => popup.calendar_date = calendar::move_right(popup.calendar_date),
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo_other(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match popup.focus {
        Focus::Todo => popup.todo.on_key_press(key_event),
        Focus::Info => popup.info.on_key_press(key_event),
        Focus::Status => {}
        Focus::DueDate => {}
        Focus::Project => popup.project.on_key_press(key_event),
    }
    DialogAction::None
}

fn update_todo(popup: &mut TodoPopup, key_event: KeyEvent) -> DialogAction {
    match key_event.code {
        // Tab changes focus
        KeyCode::Tab => {
            if key_event.modifiers == KeyModifiers::SHIFT {
                popup.focus_previous();
            } else {
                popup.focus_next();
            }
            DialogAction::None
        }
        KeyCode::Enter => update_todo_enter(popup),
        KeyCode::Char(' ') => update_todo_space(popup, key_event),
        KeyCode::Left => update_todo_left(popup),
        KeyCode::Right => update_todo_right(popup),
        KeyCode::Char('p') => update_todo_p(popup, key_event),
        KeyCode::Char('n') => update_todo_n(popup, key_event),
        KeyCode::Char('j') => update_todo_j(popup, key_event),
        KeyCode::Char('k') => update_todo_k(popup, key_event),
        KeyCode::Char('h') => update_todo_h(popup, key_event),
        KeyCode::Char('l') => update_todo_l(popup, key_event),
        _ => update_todo_other(popup, key_event),
    }
}

fn update_project(popup: &mut ProjectPopup, key_event: KeyEvent) -> DialogAction {
    // Enter submits the form
    match key_event.code {
        KeyCode::Enter => DialogAction::SubmitProject,
        KeyCode::Left => {
            popup.name.cursor_left();
            DialogAction::None
        }
        KeyCode::Right => {
            popup.name.cursor_right();
            DialogAction::None
        }
        _ => {
            popup.name.on_key_press(key_event);
            DialogAction::None
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    match &app.dialog {
        None => update_normal(app, key_event),
        Some(_) => update_edit(app, key_event),
    }
}
