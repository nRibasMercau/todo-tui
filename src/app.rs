use crate::ui::todo_popup::TodoPopup;
use crate::{
    db::{project, todo},
    models::{
        project::Project,
        todo::{NewTodoRecord, Status, Todo, TodoRecord},
    },
};
use chrono::NaiveDate;
use ratatui::widgets::ListState;
use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    conn: Connection,
    pub should_quit: bool,
    pub todo_list: TodoList,
    pub projects: Vec<Project>,
    pub popup: Option<TodoPopup>,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<Todo>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum TodoListError {
    InvalidIndex,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            should_quit: false,
            todo_list: TodoList::from_iter([
                (
                    1,
                    String::from("Learn Rust"),
                    String::from("Finish learning Rust"),
                    Status::Done,
                    Some(String::from("rust")),
                    Some(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap()),
                ),
                (
                    2,
                    String::from("Finish this app"),
                    String::from("Finish this tui list app"),
                    Status::InProgress,
                    Some(String::from("rust")),
                    None,
                ),
                (
                    3,
                    String::from("Create and push repository"),
                    String::from("Create new git repository and upload the app"),
                    Status::ToDo,
                    Some(String::from("rust")),
                    None,
                ),
            ]),
            projects: vec![Project {
                id: 1,
                name: String::from("rust"),
                archived: false,
            }],
            popup: None,
            error_message: None,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn submit_todo(&mut self) -> rusqlite::Result<()> {
        let Some(popup) = self.popup.take() else {
            return Ok(());
        };

        // Get the id of the todo from the popup
        let todo_id = popup.id;
        // Get NewTodo from the popup
        let new_todo = popup.into_new_todo();

        // Resolve project name
        // If the project exists, get the id
        // If the project doesn't exists, ask user
        let project_id = match new_todo.project.as_deref() {
            Some(project) => project::get_by_name(&self.conn, &project)?,
            None => None,
        };

        // INSERT - UPDATE
        // If popup.id is Some, it's an edit of an existing todo
        // Update the existing todo
        if let Some(todo_id) = todo_id {
            let todo = TodoRecord {
                id: todo_id,
                todo: new_todo.todo,
                info: new_todo.info,
                status: new_todo.status,
                project_id,
                due_date: new_todo.due_date,
            };
            todo::update(&self.conn, todo)?;
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
            };
            todo::create(&self.conn, todo)?;
        };

        Ok(())
        //
    }

    /// Selects next element in the list
    pub fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    /// Selects previous element in the list
    pub fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    pub fn open_todo_popup(&mut self, item: Option<usize>) {
        self.error_message = None;
        if let Some(item) = item {
            let todo_item = &self.todo_list.items[item];
            self.popup = Some(TodoPopup::from_todo(todo_item));
        } else {
            self.popup = Some(TodoPopup::new());
        }
    }

    pub fn find_project_id(&self, project_name: &str) -> Option<i64> {
        self.projects
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
            ),
        >,
    {
        let items: Vec<Todo> = iter
            .into_iter()
            .map(|(id, todo, info, status, project, due_date)| Todo {
                id,
                todo,
                info,
                status,
                project,
                due_date,
            })
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

impl TodoList {
    pub fn replace_todo(&mut self, todo_item: Todo) -> Result<(), TodoListError> {
        match self.items.iter_mut().find(|i| i.id == todo_item.id) {
            Some(item) => {
                *item = todo_item;
                Ok(())
            }
            None => Err(TodoListError::InvalidIndex),
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].status = self.items[i].status.next()
        }
    }
}
