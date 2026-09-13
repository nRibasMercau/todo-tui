use crate::ui::project_popup::ProjectPopup;
use crate::ui::todo_popup::TodoPopup;
use crate::{
    db::{project, todo},
    models::{
        project::{NewProject, Project},
        todo::{NewTodoRecord, Status, Todo, TodoRecord},
    },
};
use chrono::{Local, NaiveDate};
use ratatui::widgets::ListState;
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
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<Todo>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct ProjectList {
    pub items: Vec<Project>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum TodoListError {
    TodoNotFound,
}

#[derive(Debug)]
pub enum ProjectListError {
    ProjectNotFound,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    Todos,
    Projects,
}

#[derive(Debug)]
pub enum Dialog {
    None,
    Confirm { title: String, message: String },
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
            todo_list: TodoList::new(todos),
            projects: ProjectList::new(projects),
            dialog: None,
            error_message: None,
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

                // Resolve project name
                // If the project exists, get the id
                // TODO: If the project doesn't exists, ask user
                // For the moment, it's creating the new project by default
                let project_id = match new_todo.project.as_deref() {
                    Some(project) => match project::get_by_name(&self.conn, &project)? {
                        Some(project_id) => Some(project_id),
                        None => {
                            let new_project = project::create(
                                &mut self.conn,
                                NewProject {
                                    name: project.to_string(),
                                    archived: false,
                                },
                            )?;

                            Some(new_project.id)
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
                    let todo = todo::update(&mut self.conn, todo)?;
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
                        created_at: Local::now().date_naive(),
                        completed_at: None,
                    };
                    let todo = todo::create(&mut self.conn, todo)?;
                    self.todo_list.add_todo(todo);
                };
                self.active_panel = ActivePanel::Todos;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn toggle_status_todo(&mut self) -> rusqlite::Result<()> {
        if let Some(i) = self.todo_list.state.selected() {
            let id = self.todo_list.items[i].id;
            let new_status = self.todo_list.items[i].status.next();

            let current_todo = todo::get_by_id(&mut self.conn, id)?;
            let completed_at = match (current_todo.status, new_status) {
                (Status::ToDo, Status::Done) => Some(Local::now().date_naive()),
                (Status::InProgress, Status::Done) => Some(Local::now().date_naive()),
                (Status::Done, Status::InProgress) => None,
                (Status::Done, Status::ToDo) => None,
                _ => current_todo.completed_at,
            };

            todo::update_status(&mut self.conn, id, new_status, completed_at)?;
        }
        self.todo_list.toggle_status();
        Ok(())
    }

    pub fn delete_todo(&mut self) -> rusqlite::Result<()> {
        if let Some(i) = self.todo_list.state.selected() {
            let todo_id = self.todo_list.items[i].id;

            // Delete db record
            todo::delete(&self.conn, todo_id)?;

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
        };

        Ok(())
    }

    pub fn open_todo_popup(&mut self, item: Option<usize>) {
        self.error_message = None;
        if let Some(item) = item {
            let todo_item = &self.todo_list.items[item];
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

    pub fn find_project_id(&self, project_name: &str) -> Option<i64> {
        self.projects
            .items
            .iter()
            .find(|p| p.name == project_name)
            .map(|p| p.id)
    }
}

impl
    FromIterator<(
        i64,
        String,
        String,
        Status,
        Option<String>,
        Option<NaiveDate>,
        NaiveDate,
        Option<NaiveDate>,
    )> for TodoList
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<
            Item = (
                i64,
                String,
                String,
                Status,
                Option<String>,
                Option<NaiveDate>,
                NaiveDate,
                Option<NaiveDate>,
            ),
        >,
    {
        let items: Vec<Todo> = iter
            .into_iter()
            .map(
                |(id, todo, info, status, project, due_date, created_at, completed_at)| Todo {
                    id,
                    todo,
                    info,
                    status,
                    project,
                    due_date,
                    created_at,
                    completed_at,
                },
            )
            .collect();

        // State
        // By default, the first item of the list will be selected
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }
}

impl ProjectList {
    pub fn new(items: Vec<Project>) -> Self {
        let mut state = ListState::default();

        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }

    /// Selects next element in the list
    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    /// Selects previous element in the list
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn add_project(&mut self, project: Project) {
        self.items.push(project);
    }

    pub fn replace_project(&mut self, project: Project) -> Result<(), ProjectListError> {
        match self.items.iter_mut().find(|i| i.id == project.id) {
            Some(item) => {
                *item = project;
                Ok(())
            }
            None => Err(ProjectListError::ProjectNotFound),
        }
    }
}

impl TodoList {
    pub fn new(items: Vec<Todo>) -> Self {
        let mut state = ListState::default();

        if !items.is_empty() {
            state.select(Some(0));
        }

        Self { items, state }
    }

    /// Selects next element in the list
    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    /// Selects previous element in the list
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.items.push(todo);
    }

    pub fn replace_todo(&mut self, todo_item: Todo) -> Result<(), TodoListError> {
        match self.items.iter_mut().find(|i| i.id == todo_item.id) {
            Some(item) => {
                *item = todo_item;
                Ok(())
            }
            None => Err(TodoListError::TodoNotFound),
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].status = self.items[i].status.next()
        }
    }
}
