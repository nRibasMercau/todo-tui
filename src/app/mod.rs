pub mod project_list;
pub mod todo_table;

pub use project_list::ProjectList;
pub use todo_table::TodoTable;

use crate::app::project_list::ProjectListItem;
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
    pub todo_filter: TodoFilter,
    pub dialog: Option<Dialog>,
    pub error_message: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    Todos,
    Projects,
}

#[derive(Debug, Default)]
pub struct TodoFilter {
    pub project_id: Option<i64>,
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
        let todo_filter = TodoFilter::default();
        let todos = todo::get(&conn, &todo_filter)?;
        let projects = project::get(&conn)?;
        let project_items = std::iter::once(ProjectListItem::All)
            .chain(projects.into_iter().map(ProjectListItem::Project))
            .collect();
        Ok(Self {
            conn,
            should_quit: false,
            active_panel: ActivePanel::Todos,
            projects: ProjectList::new(project_items),
            todo_filter: todo_filter,
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

    pub fn toggle_status_todo(&mut self, todo_id: TodoId) -> rusqlite::Result<()> {
        let current_todo = todo::get_by_id(&mut self.conn, todo_id)?;
        let new_status = current_todo.status.next();

        let completed_at = match (current_todo.status, new_status) {
            (Status::ToDo, Status::Done) => Some(Local::now().date_naive()),
            (Status::InProgress, Status::Done) => Some(Local::now().date_naive()),
            (Status::Done, Status::InProgress) => None,
            (Status::Done, Status::ToDo) => None,
            _ => current_todo.completed_at,
        };

        todo::update_status(&mut self.conn, current_todo.id, new_status, completed_at)?;

        let todos = todo::get(&mut self.conn, &self.todo_filter)?;
        self.todo_table = TodoTable::new(todos);

        self.active_panel = ActivePanel::Todos;

        Ok(())
    }

    pub fn delete_todo(&mut self, todo_id: TodoId) -> rusqlite::Result<()> {
        // Delete db record
        todo::delete(&self.conn, todo_id)?;

        self.reload_todos()?;

        Ok(())
    }

    pub fn delete_project(&mut self, project_id: ProjectId) -> rusqlite::Result<()> {
        // Delete db record
        project::delete(&self.conn, project_id)?;

        // Reload project list
        self.reload_projects()?;

        // State of the project list: All selected
        // Filter: all
        self.projects.state.select(Some(0));
        self.todo_filter.project_id = None;

        // Reload todos
        self.reload_todos()?;

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
        let Some(ProjectListItem::Project(project)) = self.projects.items.get(index) else {
            return;
        };

        self.dialog = Some(Dialog::Project(ProjectPopup::from_project(project)))
    }

    /// Opens dialog for confirming deletion of project
    pub fn confirm_delete_project(&mut self, index: usize) {
        let Some(ProjectListItem::Project(project)) = self.projects.items.get(index) else {
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
        todo::create(&mut self.conn, &todo)?;

        self.reload_todos()?;
        self.reload_projects()?;

        self.active_panel = ActivePanel::Todos;
        Ok(())
    }

    pub fn update_todo(&mut self, todo_id: TodoId, new_todo: TodoFormData) -> rusqlite::Result<()> {
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
        todo::update(&mut self.conn, &todo)?;

        self.reload_todos()?;
        self.reload_projects()?;

        self.active_panel = ActivePanel::Todos;

        Ok(())
    }

    pub fn create_project(&mut self, new_project: NewProject) -> rusqlite::Result<()> {
        project::create(&mut self.conn, new_project)?;
        self.reload_projects()?;
        Ok(())
    }

    pub fn update_project(
        &mut self,
        project_id: ProjectId,
        new_project: ProjectFormData,
    ) -> rusqlite::Result<()> {
        let current_project = project::get_by_id(&mut self.conn, project_id)?;
        project::update(
            &mut self.conn,
            Project {
                id: project_id,
                name: new_project.name,
                archived: new_project.archived,
                created_at: current_project.created_at,
            },
        )?;
        self.reload_projects()?;
        Ok(())
    }

    pub fn reload_projects(&mut self) -> rusqlite::Result<()> {
        let projects = project::get(&self.conn)?;
        self.projects.replace_projects(projects);
        Ok(())
    }

    pub fn reload_todos(&mut self) -> rusqlite::Result<()> {
        let selected_id = self.todo_table.selected_todo_id();

        let todos = todo::get(&self.conn, &self.todo_filter)?;
        self.todo_table.items = todos;

        let index = selected_id
            .and_then(|id| self.todo_table.items.iter().position(|todo| todo.id == id))
            .or_else(|| {
                if self.todo_table.items.is_empty() {
                    None
                } else {
                    Some(0)
                }
            });

        self.todo_table.state.select(index);
        Ok(())
    }

    pub fn select_project(&mut self, project_id: Option<ProjectId>) -> rusqlite::Result<()> {
        self.todo_filter.project_id = project_id;

        let todos = todo::get(&self.conn, &self.todo_filter)?;
        self.todo_table.items = todos;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::migrate;
    use crate::update::update;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use rusqlite::Result;

    fn test_db(empty: bool) -> Result<Connection> {
        let mut conn = Connection::open_in_memory()?;
        migrate(&mut conn)?;
        if !empty {
            conn.execute_batch(
                "
            INSERT INTO projects (id, name, archived) VALUES (1, 'Project 1', 0);
            INSERT INTO projects (id, name, archived) VALUES (2, 'Project 2', 0);
            INSERT INTO projects (id, name, archived) VALUES (3, 'Project 3', 0);
        ",
            )?;
            conn.execute_batch("
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (1,   'Todo 1',   'todo 1',     'todo',           1,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (2,   'Todo 2',   'todo 2',     'in_progress',    1,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (3,   'Todo 3',   'todo 3',     'in_progress',    2,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (4,   'Todo 4',   'todo 4',     'todo',           2,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (5,   'Todo 5',   'todo 5',     'done',           2,     NULL,            '2026-05-01');
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (6,   'Todo 6',   'todo 6',     'todo',           3,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (7,   'Todo 7',   'todo 7',     'in_progress',    3,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (8,   'Todo 8',   'todo 8',     'in_progress',    3,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (9,   'Todo 9',   'todo 9',     'todo',           3,     NULL,            NULL);
            INSERT INTO todos (id, todo, info, status, project_id, due_date, completed_at) VALUES (10,  'Todo 10',  'todo 10',     'done',          3,     '2026-07-01',    '2026-08-01');
        ")?;
        }

        Ok(conn)
    }

    #[test]
    fn pressing_a_in_projects_opens_project_popup() -> Result<()> {
        let conn = test_db(true)?;
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
        let conn = test_db(true)?;
        let mut app = App::new(conn)?;

        app.active_panel = ActivePanel::Todos;

        update(
            &mut app,
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        );

        assert!(matches!(app.dialog, Some(Dialog::Todo(_))));

        Ok(())
    }

    #[test]
    fn pressing_enter_in_todos_opens_todo_popup() -> Result<()> {
        let conn = test_db(false)?;
        let mut app = App::new(conn)?;

        app.active_panel = ActivePanel::Todos;
        app.todo_table.state.select(Some(1));

        update(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert!(matches!(app.dialog, Some(Dialog::Todo(_))));

        Ok(())
    }

    #[test]
    fn pressing_enter_in_projects_opens_project_popup() -> Result<()> {
        let conn = test_db(false)?;
        let mut app = App::new(conn)?;

        app.active_panel = ActivePanel::Projects;
        app.projects.state.select(Some(1));

        update(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert!(matches!(app.dialog, Some(Dialog::Project(_))));

        Ok(())
    }

    #[test]
    fn selecting_a_project_filters_todos() -> Result<()> {
        let conn = test_db(false)?;
        let mut app = App::new(conn)?;

        app.select_project(Some(1))?;
        assert_eq!(app.todo_filter.project_id, Some(1));
        let mut actual: Vec<_> = app.todo_table.items.iter().map(|todo| todo.id).collect();
        actual.sort();
        assert_eq!(actual, vec![1, 2]);

        app.select_project(Some(3))?;
        assert_eq!(app.todo_filter.project_id, Some(3));
        actual = app.todo_table.items.iter().map(|todo| todo.id).collect();
        actual.sort();
        assert_eq!(actual, vec![6, 7, 8, 9, 10]);

        Ok(())
    }

    #[test]
    fn submitting_todo_creates_todo() -> Result<()> {
        let conn = test_db(true)?;
        let mut app = App::new(conn)?;

        app.create_todo(NewTodo {
            todo: String::from("My todo"),
            info: String::from("My todo info"),
            status: Status::ToDo,
            project: Some(String::from("My project")),
            due_date: None,
        })?;

        let todo = &app.todo_table.items[0];
        let todo_id = todo.id;
        let todo_project_id = todo.project_id;
        assert_eq!(todo_id, 1);
        assert_eq!(todo_project_id, Some(1));

        let project_id = match &app.projects.items[1] {
            ProjectListItem::Project(project) => project.id,
            ProjectListItem::All => panic!("expected a project"),
        };
        assert_eq!(project_id, 1);

        Ok(())
    }

    #[test]
    fn submitting_project_creates_project() -> Result<()> {
        let conn = test_db(true)?;
        let mut app = App::new(conn)?;

        app.create_project(NewProject {
            name: String::from("My project"),
        })?;

        let project_id = match &app.projects.items[1] {
            ProjectListItem::Project(project) => project.id,
            ProjectListItem::All => panic!("expected a project"),
        };
        assert_eq!(project_id, 1);

        Ok(())
    }

    #[test]
    fn submitting_project_edits_existing_project() -> Result<()> {
        let conn = test_db(true)?;
        let mut app = App::new(conn)?;

        app.create_project(NewProject {
            name: String::from("My project"),
        })?;

        let project_id = match &app.projects.items[1] {
            ProjectListItem::Project(project) => project.id,
            ProjectListItem::All => panic!("expected a project"),
        };

        let project_name = match &app.projects.items[1] {
            ProjectListItem::Project(project) => &project.name,
            ProjectListItem::All => panic!("expected a project"),
        };

        assert_eq!(project_name, "My project");

        app.update_project(
            project_id,
            ProjectFormData {
                name: String::from("My updated project"),
                archived: false,
            },
        )?;

        let project_name = match &app.projects.items[1] {
            ProjectListItem::Project(project) => &project.name,
            ProjectListItem::All => panic!("expected a project"),
        };

        assert_eq!(project_name, "My updated project");

        Ok(())
    }
}
