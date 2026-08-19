use ratatui::widgets::ListState;
use std::fmt;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub todo_list: TodoList,
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct TodoItem {
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub tag: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            todo_list: TodoList::from_iter([
                (Status::Done, "Learn Rust", "Finish learning Rust", "rust"),
                (
                    Status::InProgress,
                    "Finish this app",
                    "Finish this tui todo list app",
                    "rust",
                ),
                (
                    Status::ToDo,
                    "Create and push repositiory",
                    "Create new git repository and upload the app",
                    "rust",
                ),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str, &'static str)> for TodoList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Status, &'static str, &'static str, &'static str)>,
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
    pub fn new(status: Status, todo: &str, info: &str, tag: &str) -> Self {
        Self {
            status: status,
            todo: todo.to_string(),
            info: info.to_string(),
            tag: tag.to_string(),
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
