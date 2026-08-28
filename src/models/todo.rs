use chrono::NaiveDate;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::ToDo => write!(f, "To Do"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug)]
pub struct TodoItem {
    pub id: i64,
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project_id: Option<i64>,
    pub due_date: Option<NaiveDate>,
}

impl TodoItem {
    pub fn toggle_status(&mut self) {
        self.status = self.status.next();
    }
}

#[derive(Debug)]
pub struct NewTodoItem {
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project_id: Option<i64>,
    pub due_date: Option<NaiveDate>,
}

impl NewTodoItem {
    pub fn new(
        status: Status,
        todo: String,
        info: String,
        project_id: Option<i64>,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            status,
            todo,
            info,
            project_id,
            due_date,
        }
    }
}
