use chrono::NaiveDate;
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};
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

    pub fn as_str(&self) -> &'static str {
        match self {
            Status::ToDo => "todo",
            Status::InProgress => "in_progress",
            Status::Done => "done",
        }
    }

    pub fn from_str(status: &str) -> Result<Self, String> {
        match status {
            "todo" => Ok(Status::ToDo),
            "in_progress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {status}")),
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

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        let status = value.as_str()?;
        Status::from_str(status).map_err(|err| FromSqlError::Other(err.into()))
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.as_str().to_owned())))
    }
}

/*
 * Represents the db record
 */
#[derive(Debug)]
pub struct TodoRecord {
    pub id: i64,
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project_id: Option<i64>,
    pub due_date: Option<NaiveDate>,
}

/*
 * Represents the todo as shown in the list
 */
#[derive(Debug)]
pub struct Todo {
    pub id: i64,
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project: Option<String>,
    pub due_date: Option<NaiveDate>,
}

impl Todo {
    pub fn toggle_status(&mut self) {
        self.status = self.status.next();
    }
}

/*
 * Information to create a new todo
 */
#[derive(Debug)]
pub struct NewTodo {
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project: Option<String>,
    pub due_date: Option<NaiveDate>,
}

impl NewTodo {
    pub fn new(
        status: Status,
        todo: String,
        info: String,
        project: Option<String>,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            status,
            todo,
            info,
            project,
            due_date,
        }
    }
}

/*
 * Information to create a new todo in db
 */
#[derive(Debug)]
pub struct NewTodoRecord {
    pub todo: String,
    pub info: String,
    pub status: Status,
    pub project_id: Option<i64>,
    pub due_date: Option<NaiveDate>,
}
