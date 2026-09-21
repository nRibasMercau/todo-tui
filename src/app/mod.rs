pub mod project_list;
pub mod todo_table;

pub use project_list::ProjectList;
pub use todo_table::TodoTable;

use crate::models::todo::TodoFormData;
use crate::ui::confirm_popup::{ConfirmAction, ConfirmChoice, ConfirmPopup};
use crate::ui::project_popup::ProjectPopup;
use crate::ui::todo_popup::TodoPopup;
use crate::{
    db::{project, todo},
    models::{
        project::{NewProject, Project, ProjectFormData, ProjectId},
        todo::{NewTodo, NewTodoRecord, Status, TodoId, TodoRecord},
    },
};
use chrono::Local;
use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    conn: Connection,
    pub should_quit: bool,
    pub active_panel: ActivePanel,
    pub todo_table: TodoTable,
    pub projects: ProjectList,
    pub dialog: Option<Dialog>,
    pub error_message: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    Todos,
    Projects,
}

#[derive(Debug)]
pub enum Dialog {
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
        let updated_todo = todo::get_by_id(&mut self.conn, todo.id)?;
        match self.todo_table.replace_todo(updated_todo) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("replace todo failed: {err:?}");
                eprintln!("table: {:?}", self.todo_table.items);
            }
        }

        Ok(())
    }

    pub fn delete_todo(&mut self, todo_id: TodoId) -> rusqlite::Result<()> {
        // Delete db record
        todo::delete(&self.conn, todo_id)?;

        // Find index of the deleted todo in the current list
        // before removing it
        let i = self
            .todo_table
            .items
            .iter()
            .position(|todo| todo.id == todo_id)
            .unwrap_or(0);

        // Remove item from the list
        self.todo_table.items.retain(|todo| todo.id != todo_id);

        // Handle selection status
        // If there are no todos, select is None
        if self.todo_table.items.is_empty() {
            self.todo_table.state.select(None);
            // If the deleted todo was the last one in the list,
            // select the new last one
        } else if i >= self.todo_table.items.len() {
            self.todo_table
                .state
                .select(Some(self.todo_table.items.len() - 1));
        // Otherwise, select the new i
        } else {
            self.todo_table.state.select(Some(i));
        }

        Ok(())
    }

    pub fn delete_project(&mut self, project_id: ProjectId) -> rusqlite::Result<()> {
        // Delete db record
        project::delete(&self.conn, project_id)?;

        // Find index of the deleted project in the current list
        // before removing it
        let i = self
            .projects
            .items
            .iter()
            .position(|project| project.id == project_id)
            .unwrap_or(0);

        // Remove project from the list
        self.projects
            .items
            .retain(|project| project.id != project_id);

        // Remove todos from the todo list
        self.todo_table
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

    /// Opens dialog for new todo
    pub fn add_todo(&mut self) {
        self.dialog = Some(Dialog::Todo(TodoPopup::new()));
    }

    /// Opens dialog for editing existing todo
    pub fn edit_todo(&mut self, index: usize) {
        let Some(todo) = self.todo_table.items.get(index) else {
            return;
        };

        self.dialog = Some(Dialog::Todo(TodoPopup::from_todo(todo)))
    }

    /// Opens dialog for confirming deletion of todo
    pub fn confirm_delete_todo(&mut self, index: usize) {
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

    /// Opens dialog for new project
    pub fn add_project(&mut self) {
        self.dialog = Some(Dialog::Project(ProjectPopup::new()));
    }

    /// Opens dialog for editing existing project
    pub fn edit_project(&mut self, index: usize) {
        let Some(project) = self.projects.items.get(index) else {
            return;
        };

        self.dialog = Some(Dialog::Project(ProjectPopup::from_project(project)))
    }

    /// Opens dialog for confirming deletion of project
    pub fn confirm_delete_project(&mut self, index: usize) {
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

    pub fn close_dialog(&mut self) {
        self.dialog = None;
    }

    pub fn create_todo(&mut self, todo: NewTodo) -> rusqlite::Result<()> {
        let mut new_project: Option<Project> = None;
        // Resolve project name
        // If the project exists, get the id
        // TODO: If the project doesn't exists, ask user
        // For the moment, it's creating the new project by default
        let project_id = match todo.project.as_deref() {
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

        let todo = NewTodoRecord {
            todo: todo.todo,
            info: todo.info,
            status: todo.status,
            project_id,
            due_date: todo.due_date,
            completed_at: None,
        };
        let created_todo = todo::create(&mut self.conn, &todo)?;
        // Add todo to the list
        self.todo_table.add_todo(created_todo);
        // Add new project to the list, only if a new project was created
        if let Some(project) = new_project {
            self.projects.add_project(project);
        }
        self.active_panel = ActivePanel::Todos;
        Ok(())
    }

    pub fn update_todo(&mut self, todo_id: TodoId, new_todo: TodoFormData) -> rusqlite::Result<()> {
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
        match self.todo_table.replace_todo(todo) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("replace todo failed: {err:?}");
                eprintln!("table: {:?}", self.todo_table.items);
            }
        }

        // Add new project to the list, only if a new project was created
        if let Some(project) = new_project {
            self.projects.add_project(project);
        }
        Ok(())
    }

    pub fn create_project(&mut self, new_project: NewProject) -> rusqlite::Result<()> {
        let project = project::create(&mut self.conn, new_project)?;
        self.projects.add_project(project);
        Ok(())
    }

    pub fn update_project(
        &mut self,
        project_id: ProjectId,
        new_project: ProjectFormData,
    ) -> rusqlite::Result<()> {
        let current_project = project::get_by_id(&mut self.conn, project_id)?;
        let project = project::update(
            &mut self.conn,
            Project {
                id: project_id,
                name: new_project.name,
                archived: new_project.archived,
                created_at: current_project.created_at,
            },
        )?;
        self.projects
            .replace_project(project)
            .expect("Internal error: updated must exist in ProjectList");

        Ok(())
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
