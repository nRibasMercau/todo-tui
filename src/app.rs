use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Offset, widgets::ListState};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub todo_list: TodoList,
    pub popup: Option<TodoPopup>,
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct TodoItem {
    pub todo: StringField,
    pub info: StringField,
    pub status: Status,
    pub tag: StringField,
}

#[derive(Debug, Serialize, Clone)]
pub struct StringField {
    #[serde(skip)]
    pub label: &'static str,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Todo,
    Info,
    Status,
    Tag,
}

#[derive(Serialize, Debug)]
pub struct TodoPopup {
    pub todo: StringField,
    pub info: StringField,
    #[serde(skip)]
    pub status: Status,
    pub tag: StringField,
    #[serde(skip)]
    pub focus: Focus,
}

impl StringField {
    fn new(label: &'static str, value: impl Into<String>) -> Self {
        Self {
            label,
            value: value.into(),
        }
    }

    /// Handle input events for string input
    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }

    pub fn cursor_offset(&self) -> Offset {
        let x = (self.label.len() + self.value.len()) as i32;
        Offset::new(x, 0)
    }
}

impl fmt::Display for StringField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::ToDo => write!(f, "To Do"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

impl TodoPopup {
    fn new() -> Self {
        Self {
            todo: StringField::new("To do", ""),
            info: StringField::new("Description", ""),
            tag: StringField::new("Proyect", ""),
            status: Status::ToDo,
            focus: Focus::Todo,
        }
    }

    fn from_todo(todo: &TodoItem) -> Self {
        Self {
            todo: StringField::new("To do", todo.todo.to_string()),
            info: StringField::new("Description", todo.info.to_string()),
            tag: StringField::new("Proyect", todo.tag.to_string()),
            status: todo.status.clone(),
            focus: Focus::Todo,
        }
    }

    pub fn focus_next(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Info,
            Focus::Info => self.focus = Focus::Status,
            Focus::Status => self.focus = Focus::Tag,
            Focus::Tag => self.focus = Focus::Todo,
        }
    }

    pub fn focus_previous(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Tag,
            Focus::Info => self.focus = Focus::Todo,
            Focus::Status => self.focus = Focus::Info,
            Focus::Tag => self.focus = Focus::Status,
        }
    }
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

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].status = match self.todo_list.items[i].status {
                Status::ToDo => Status::InProgress,
                Status::InProgress => Status::Done,
                Status::Done => Status::ToDo,
            }
        }
    }

    pub fn open_todo_popup(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            let item = &self.todo_list.items[i];

            self.popup = Some(TodoPopup::from_todo(item));
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
                    StringField::new("tag", "rust"),
                ),
                (
                    Status::InProgress,
                    StringField::new("todo", "Finish this app"),
                    StringField::new("info", "Finish this tui list app"),
                    StringField::new("tag", "rust"),
                ),
                (
                    Status::ToDo,
                    StringField::new("todo", "Create and push repository"),
                    StringField::new("info", "Create new git repository and upload the app"),
                    StringField::new("tag", "rust"),
                ),
            ]),
            popup: None,
        }
    }
}

impl FromIterator<(Status, StringField, StringField, StringField)> for TodoList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Status, StringField, StringField, StringField)>,
    {
        let items: Vec<TodoItem> = iter
            .into_iter()
            .map(|(status, todo, info, tag)| TodoItem::new(status, todo, info, tag))
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

impl TodoItem {
    pub fn new(status: Status, todo: StringField, info: StringField, tag: StringField) -> Self {
        Self {
            status: status,
            todo: todo,
            info: info,
            tag: tag,
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = match self.status {
            Status::ToDo => Status::InProgress,
            Status::InProgress => Status::Done,
            Status::Done => Status::ToDo,
        }
    }
}
