use crate::models::todo::{Status, TodoItem};
use crate::ui::{fields::StringField, todo_popup::TodoPopup};
use chrono::NaiveDate;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub todo_list: TodoList,
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
            self.popup = Some(TodoPopup::from_todo(todo_item, item));
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
                    Status::Done,
                    StringField::new("todo", "Learn Rust"),
                    StringField::new("info", "Finish learning Rust"),
                    StringField::new("project", "rust"),
                    Some(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap()),
                ),
                (
                    Status::InProgress,
                    StringField::new("todo", "Finish this app"),
                    StringField::new("info", "Finish this tui list app"),
                    StringField::new("project", "rust"),
                    None,
                ),
                (
                    Status::ToDo,
                    StringField::new("todo", "Create and push repository"),
                    StringField::new("info", "Create new git repository and upload the app"),
                    StringField::new("project", "rust"),
                    None,
                ),
            ]),
            popup: None,
            error_message: None,
        }
    }
}

impl
    FromIterator<(
        Status,
        StringField,
        StringField,
        StringField,
        Option<NaiveDate>,
    )> for TodoList
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<
            Item = (
                Status,
                StringField,
                StringField,
                StringField,
                Option<NaiveDate>,
            ),
        >,
    {
        let items: Vec<TodoItem> = iter
            .into_iter()
            .map(|(status, todo, info, project, due_date)| {
                TodoItem::new(status, todo, info, project, due_date)
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
    pub fn replace_todo(&mut self, todo_item: TodoItem, index: usize) -> Result<(), TodoListError> {
        match self.items.get_mut(index) {
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

    pub fn add_todo(&mut self, todo_item: TodoItem) {
        self.items.push(todo_item)
    }
}
