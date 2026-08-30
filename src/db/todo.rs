use crate::models::todo::{NewTodoItem, Status, TodoItem};
use chrono::NaiveDate;
use rusqlite::{Connection, Result};

/// Creates a new todo and returns the generated ID.
pub fn create_todo(conn: &Connection, todo: &NewTodoItem) -> Result<i64> {
    let status = todo.status.as_str();
    conn.execute(
        "
        INSERT INTO todos (todo, info, status, project_id, due_date) 
        VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &todo.todo,
            &todo.info,
            status,
            &todo.project_id,
            &todo.due_date,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

/// Gets todos.
pub fn get_todos(conn: &Connection) -> Result<Vec<TodoItem>> {
    let mut stmt =
        conn.prepare("SELECT id, todo, info, status, project_id, due_date FROM todos")?;
    let todos = stmt
        .query_map([], |row| {
            let status_str: String = row.get("status")?;
            let status = Status::from_str(&status_str).map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::other(err)),
                )
            })?;
            let due_date: Option<NaiveDate> = row.get("due_date")?;
            Ok(TodoItem {
                id: row.get("id")?,
                todo: row.get("todo")?,
                info: row.get("info")?,
                status,
                project_id: row.get("project_id")?,
                due_date,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(todos)
}

/// Gets todo by ID.
pub fn get_todo_by_id(conn: &Connection) -> Result<()> {
    unimplemented!()
}

/// Updates an existing todo.
pub fn update_todo(conn: &Connection) -> Result<()> {
    unimplemented!();
}

/// Deletes a todo by ID.
pub fn delete_todo(conn: &Connection) -> Result<()> {
    unimplemented!();
}
