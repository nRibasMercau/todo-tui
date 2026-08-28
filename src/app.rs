use crate::models::{
    project::Project,
    todo::{NewTodoItem, Status, TodoItem},
};
use crate::ui::todo_popup::TodoPopup;
use chrono::NaiveDate;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub todo_list: TodoList,
    pub projects: Vec<Project>,
    pub popup: Option<TodoPopup>,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum TodoListError {
    InvalidIndex,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_to_do(&mut self, new_item: TodoItem) {
        self.todo_list.items.push(new_item)
    }

    pub fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    pub fn open_todo_popup(&mut self, item: Option<usize>) {
        self.error_message = None;
        if let Some(item) = item {
            let todo_item = &self.todo_list.items[item];
            self.popup = Some(TodoPopup::from_todo(todo_item, item, &self));
        } else {
            self.popup = Some(TodoPopup::new());
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            todo_list: TodoList::from_iter([
                (
                    1,
                    String::from("Learn Rust"),
                    String::from("Finish learning Rust"),
                    Status::Done,
                    Some(1),
                    Some(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap()),
                ),
                (
                    2,
                    String::from("Finish this app"),
                    String::from("Finish this tui list app"),
                    Status::InProgress,
                    Some(1),
                    None,
                ),
                (
                    3,
                    String::from("Create and push repository"),
                    String::from("Create new git repository and upload the app"),
                    Status::ToDo,
                    Some(1),
                    None,
                ),
            ]),
            projects: vec![Project {
                id: 1,
                name: String::from("rust"),
                archive: false,
            }],
            popup: None,
            error_message: None,
        }
    }
}

impl FromIterator<(i64, String, String, Status, Option<i64>, Option<NaiveDate>)> for TodoList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (i64, String, String, Status, Option<i64>, Option<NaiveDate>)>,
    {
        let items: Vec<TodoItem> = iter
            .into_iter()
            .map(|(id, todo, info, status, project_id, due_date)| TodoItem {
                id,
                todo,
                info,
                status,
                project_id,
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
    pub fn replace_todo(&mut self, todo_item: TodoItem) -> Result<(), TodoListError> {
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

    pub fn add_todo(&mut self, todo_item: NewTodoItem) {
        let new_todo_item = TodoItem {
            id: 50,
            todo: todo_item.todo,
            info: todo_item.info,
            status: todo_item.status,
            project_id: todo_item.project_id,
            due_date: todo_item.due_date,
        };
        self.items.push(new_todo_item);
    }
}
