pub mod project_list;
pub mod todo_list;
pub mod todo_table;

pub use project_list::ProjectList;
pub use todo_list::TodoList;
pub use todo_table::TodoTable;

use crate::ui::confirm_popup::{ConfirmAction, ConfirmChoice, ConfirmPopup};
use crate::ui::project_popup::ProjectPopup;
use crate::ui::todo_popup::TodoPopup;
use crate::{
    db::{project, todo},
    models::{
        project::{NewProject, Project, ProjectId},
        todo::{NewTodoRecord, Status, TodoId, TodoRecord},
    },
};
use chrono::Local;
use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    conn: Connection,
    pub should_quit: bool,
    pub active_panel: ActivePanel,
    pub todo_list: TodoList,
    pub projects: ProjectList,
    pub dialog: Option<Dialog>,
    pub error_message: Option<String>,
    pub todo_table: TodoTable,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    Todos,
    Projects,
}

#[derive(Debug)]
pub enum Dialog {
    None,
    Confirm(ConfirmPopup),
    Todo(TodoPopup),
    Project(ProjectPopup),
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(conn: Connection) -> rusqlite::Result<Self> {
        let todos = todo::get(&conn)?;
        let projects = project::get(&conn)?;
        tracing::info!(count = todos.len(), "fetched todos");
        tracing::info!(count = projects.len(), "fetched projects");
        Ok(Self {
            conn,
            should_quit: false,
            active_panel: ActivePanel::Todos,
            todo_list: TodoList::new(vec![]),
            projects: ProjectList::new(projects),
            dialog: None,
            error_message: None,
            todo_table: TodoTable::new(todos),
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn submit_project(&mut self) -> rusqlite::Result<()> {
        let Some(dialog) = self.dialog.take() else {
            return Ok(());
        };

        match dialog {
            Dialog::Project(popup) => {
                if let Some(project_id) = popup.id {
                    let new_project = popup.into_new_project();
                    let current_project = project::get_by_id(&mut self.conn, project_id)?;
                    let project = project::update(
                        &mut self.conn,
                        Project {
                            id: current_project.id,
                            name: new_project.name,
                            archived: false,
                            created_at: Local::now().date_naive(),
                        },
                    )?;
                    self.projects
                        .replace_project(project)
                        .expect("Internal error: updated must exist in ProjectList");
                } else {
                    let new_project = popup.into_new_project();
                    let project = project::create(&mut self.conn, new_project)?;
                    self.projects.add_project(project);
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn submit_todo(&mut self) -> rusqlite::Result<()> {
        let Some(dialog) = self.dialog.take() else {
            return Ok(());
        };

        match dialog {
            Dialog::Todo(popup) => {
                // Get the id of the todo from the popup
                let todo_id = popup.id;

                // Get NewTodo from the popup
                let new_todo = popup.into_new_todo();

                let mut new_project: Option<Project> = None;

                // Resolve project name
                // If the project exists, get the id
                // TODO: If the project doesn't exists, ask user
                // For the moment, it's creating the new project by default
                let project_id = match new_todo.project.as_deref() {
                    Some(project) => match project::get_by_name(&self.conn, &project)? {
                        Some(project_id) => Some(project_id),
                        None => {
                            let project = project::create(
                                &mut self.conn,
                                NewProject {
                                    name: project.to_string(),
                                },
                            )?;
                            let project_id = project.id;

                            new_project = Some(project);

                            Some(project_id)
                        }
                    },
                    None => None,
                };

                // INSERT - UPDATE
                // If popup.id is Some, it's an edit of an existing todo
                // Update the existing todo
                if let Some(todo_id) = todo_id {
                    let current_todo = todo::get_by_id(&mut self.conn, todo_id)?;
                    let completed_at = match (current_todo.status, new_todo.status) {
                        (Status::ToDo, Status::Done) => Some(Local::now().date_naive()),
                        (Status::InProgress, Status::Done) => Some(Local::now().date_naive()),
                        (Status::Done, Status::InProgress) => None,
                        (Status::Done, Status::ToDo) => None,
                        _ => current_todo.completed_at,
                    };

                    let todo = TodoRecord {
                        id: todo_id,
                        todo: new_todo.todo,
                        info: new_todo.info,
                        status: new_todo.status,
                        project_id,
                        due_date: new_todo.due_date,
                        created_at: current_todo.created_at,
                        completed_at,
                    };
                    let todo = todo::update(&mut self.conn, &todo)?;
                    self.todo_list
                        .replace_todo(todo)
                        .expect("Internal error: updated must exist in TodoList");

                // If popup.id is None, it's a new todo
                // Insert the new todo
                } else {
                    // Build TodoRecord
                    let todo = NewTodoRecord {
                        todo: new_todo.todo,
                        info: new_todo.info,
                        status: new_todo.status,
                        project_id,
                        due_date: new_todo.due_date,
                        completed_at: None,
                    };
                    let todo = todo::create(&mut self.conn, &todo)?;
                    // Add todo to the list
                    self.todo_list.add_todo(todo);
                    // Add new project to the list, only if a new project was created
                    if let Some(project) = new_project {
                        self.projects.add_project(project);
                    }
                };
                self.active_panel = ActivePanel::Todos;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn toggle_status_todo(&mut self, item: Option<usize>) -> rusqlite::Result<()> {
        let Some(index) = item else {
            return Ok(());
        };

        let todo = &self.todo_table.items[index];

        let current_todo = todo::get_by_id(&mut self.conn, todo.id)?;
        let new_status = current_todo.status.next();

        let completed_at = match (current_todo.status, new_status) {
            (Status::ToDo, Status::Done) => Some(Local::now().date_naive()),
            (Status::InProgress, Status::Done) => Some(Local::now().date_naive()),
            (Status::Done, Status::InProgress) => None,
            (Status::Done, Status::ToDo) => None,
            _ => current_todo.completed_at,
        };

        todo::update_status(&mut self.conn, current_todo.id, new_status, completed_at)?;
        self.todo_table.toggle_status(index);
        Ok(())
    }

    pub fn delete_todo(&mut self, todo_id: TodoId) -> rusqlite::Result<()> {
        // Delete db record
        todo::delete(&self.conn, todo_id)?;

        // Find index of the deleted todo in the current list
        // before removing it
        let i = self.todo_list.state.selected().unwrap_or(0);

        // Remove item from the list
        self.todo_list.items.retain(|todo| todo.id != todo_id);

        // Handle selection status
        // If there are no todos, select is None
        if self.todo_list.items.is_empty() {
            self.todo_list.state.select(None);
            // If the deleted todo was the last one in the list,
            // select the new last one
        } else if i >= self.todo_list.items.len() {
            self.todo_list
                .state
                .select(Some(self.todo_list.items.len() - 1));
        // Otherwise, select the new i
        } else {
            self.todo_list.state.select(Some(i));
        }

        Ok(())
    }

    pub fn delete_project(&mut self, project_id: ProjectId) -> rusqlite::Result<()> {
        // Delete db record
        project::delete(&self.conn, project_id)?;

        // Find index of the deleted project in the current list
        // before removing it
        let i = self.projects.state.selected().unwrap_or(0);

        // Remove project from the list
        self.projects
            .items
            .retain(|project| project.id != project_id);

        // Remove todos from the todo list
        self.todo_list
            .items
            .retain(|todo| todo.project_id != Some(project_id));

        // Handle selection status
        // If there are no todos, select is None
        if self.projects.items.is_empty() {
            self.projects.state.select(None);
            // If the deleted todo was the last one in the list,
            // select the new last one
        } else if i >= self.projects.items.len() {
            self.projects
                .state
                .select(Some(self.projects.items.len() - 1));
        // Otherwise, select the new i
        } else {
            self.projects.state.select(Some(i));
        }

        Ok(())
    }

    pub fn open_todo_popup(&mut self, item: Option<usize>) {
        self.error_message = None;
        if let Some(item) = item {
            let todo_item = &self.todo_table.items[item];
            self.dialog = Some(Dialog::Todo(TodoPopup::from_todo(todo_item)));
        } else {
            self.dialog = Some(Dialog::Todo(TodoPopup::new()));
        }
    }

    pub fn open_project_popup(&mut self, project_id: Option<usize>) {
        self.error_message = None;
        if let Some(project_id) = project_id {
            let project = &self.projects.items[project_id];
            self.dialog = Some(Dialog::Project(ProjectPopup::from_project(project)));
        } else {
            self.dialog = Some(Dialog::Project(ProjectPopup::new()));
        }
    }

    pub fn open_confirm_delete_todo(&mut self, index: usize) {
        let Some(todo) = self.todo_table.items.get(index) else {
            return;
        };

        self.dialog = Some(Dialog::Confirm(ConfirmPopup {
            title: "Delete todo".into(),
            message: format!("Delete task \"{}\"?", todo.todo),
            action: ConfirmAction::DeleteTodo(todo.id),
            selected: ConfirmChoice::No,
        }));
    }

    pub fn open_confirm_delete_project(&mut self, index: usize) {
        let Some(project) = self.projects.items.get(index) else {
            return;
        };

        self.dialog = Some(Dialog::Confirm(ConfirmPopup {
            title: "Delete project".into(),
            message: format!("Delete project\"{}\"?", project.name),
            action: ConfirmAction::DeleteProject(project.id),
            selected: ConfirmChoice::No,
        }));
    }

    pub fn find_project_id(&self, project_name: &str) -> Option<i64> {
        self.projects
            .items
            .iter()
            .find(|p| p.name == project_name)
            .map(|p| p.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::migrate;
    use crate::update::update;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use rusqlite::Result;

    fn test_db() -> Result<Connection> {
        let mut conn = Connection::open_in_memory()?;
        migrate(&mut conn)?;
        Ok(conn)
    }

    #[test]
    fn pressing_a_in_projects_opens_project_popup() -> Result<()> {
        let conn = test_db()?;
        let mut app = App::new(conn)?;

        app.active_panel = ActivePanel::Projects;

        update(
            &mut app,
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        );

        assert!(matches!(app.dialog, Some(Dialog::Project(_))));

        Ok(())
    }

    #[test]
    fn pressing_a_in_todos_opens_todo_popup() -> Result<()> {
        let conn = test_db()?;
        let mut app = App::new(conn)?;

        app.active_panel = ActivePanel::Todos;

        update(
            &mut app,
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        );

        assert!(matches!(app.dialog, Some(Dialog::Todo(_))));

        Ok(())
    }
}
