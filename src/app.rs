use chrono::NaiveDate;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Offset, Position, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, ListState, Padding},
};
use std::fmt;

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
pub struct TodoItem {
    pub todo: StringField,
    pub info: StringField,
    pub status: Status,
    pub proyect: StringField,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct StringField {
    pub label: &'static str,
    pub value: String,
    pub cursor: usize,
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
    Proyect,
}

#[derive(Debug)]
pub struct TodoPopup {
    pub todo: StringField,
    pub info: StringField,
    pub status: Status,
    pub proyect: StringField,
    pub due_date: Option<NaiveDate>,
    pub focus: Focus,
    pub editing: Option<usize>,
}

#[derive(Debug)]
pub enum TodoListError {
    InvalidIndex,
}

impl Status {
    pub fn next(&self) -> Self {
        match self {
            Status::ToDo => Status::InProgress,
            Status::InProgress => Status::Done,
            Status::Done => Status::ToDo,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Status::ToDo => Status::Done,
            Status::InProgress => Status::ToDo,
            Status::Done => Status::InProgress,
        }
    }
}

impl StringField {
    pub fn new(label: &'static str, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            label,
            cursor: value.len(),
            value,
        }
    }

    pub fn blank(label: &'static str) -> Self {
        Self {
            label,
            value: String::new(),
            cursor: 0,
        }
    }

    /// Handle input events for string input
    pub fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            _ => {}
        }
    }

    pub fn cursor_offset(&self) -> Offset {
        let x = (self.label.len() + self.value.len()) as i32;
        Offset::new(x, 0)
    }

    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    pub fn cursor_position(&self, area: Rect) -> Position {
        #[allow(unused_variables)]
        let [label_area, value_area] = area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ]));

        let value_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default())
            .padding(Padding::horizontal(1));

        let value_inner = value_block.inner(value_area);

        Position {
            x: value_inner.x + self.cursor as u16,
            y: value_inner.y,
        }
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
            todo: StringField::blank("To do"),
            info: StringField::blank("Description"),
            proyect: StringField::blank("Proyect"),
            status: Status::ToDo,
            due_date: None,
            focus: Focus::Todo,
            editing: None,
        }
    }

    fn from_todo(todo: &TodoItem, index: usize) -> Self {
        Self {
            todo: StringField::new("To do", todo.todo.to_string()),
            info: StringField::new("Description", todo.info.to_string()),
            proyect: StringField::new("Proyect", todo.proyect.to_string()),
            status: todo.status,
            due_date: todo.due_date,
            focus: Focus::Todo,
            editing: Some(index),
        }
    }

    pub fn focus_next(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Info,
            Focus::Info => self.focus = Focus::Status,
            Focus::Status => self.focus = Focus::Proyect,
            Focus::Proyect => self.focus = Focus::Todo,
        }
    }

    pub fn focus_previous(&mut self) {
        match &self.focus {
            Focus::Todo => self.focus = Focus::Proyect,
            Focus::Info => self.focus = Focus::Todo,
            Focus::Status => self.focus = Focus::Info,
            Focus::Proyect => self.focus = Focus::Status,
        }
    }

    pub fn submit(&mut self) -> TodoItem {
        TodoItem {
            todo: StringField::new("To do", self.todo.to_string()),
            info: StringField::new("Description", self.info.to_string()),
            status: self.status,
            proyect: StringField::new("Proyect", self.proyect.to_string()),
            due_date: self.due_date,
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
                    StringField::new("proyect", "rust"),
                    Some(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap()),
                ),
                (
                    Status::InProgress,
                    StringField::new("todo", "Finish this app"),
                    StringField::new("info", "Finish this tui list app"),
                    StringField::new("proyect", "rust"),
                    None,
                ),
                (
                    Status::ToDo,
                    StringField::new("todo", "Create and push repository"),
                    StringField::new("info", "Create new git repository and upload the app"),
                    StringField::new("proyect", "rust"),
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
            .map(|(status, todo, info, proyect, due_date)| {
                TodoItem::new(status, todo, info, proyect, due_date)
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

impl TodoItem {
    pub fn new(
        status: Status,
        todo: StringField,
        info: StringField,
        proyect: StringField,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            status: status,
            todo: todo,
            info: info,
            proyect: proyect,
            due_date: due_date,
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = self.status.next();
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
