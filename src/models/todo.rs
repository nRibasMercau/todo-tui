use crate::ui::fields::StringField;
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
    pub todo: StringField,
    pub info: StringField,
    pub status: Status,
    pub project: StringField,
    pub due_date: Option<NaiveDate>,
}

impl TodoItem {
    pub fn new(
        status: Status,
        todo: StringField,
        info: StringField,
        project: StringField,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            status: status,
            todo: todo,
            info: info,
            project: project,
            due_date: due_date,
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = self.status.next();
    }
}
