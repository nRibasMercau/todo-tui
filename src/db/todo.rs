use crate::models::todo::{NewTodoRecord, Status, Todo, TodoRecord};
use chrono::Local;
use rusqlite::{Connection, Result, params};

/// Creates a new todo and returns the new todo.
pub fn create(conn: &mut Connection, todo: NewTodoRecord) -> Result<Todo> {
    let status = todo.status.as_str();

    tracing::debug!("inserting todo");
    let tx = conn.transaction()?;
    tx.execute(
        "
        INSERT INTO todos (todo, info, status, project_id, due_date, created_at, completed_at)
        VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_DATE, NULL)",
        (
            &todo.todo,
            &todo.info,
            status,
            &todo.project_id,
            &todo.due_date,
        ),
    )?;

    let id = tx.last_insert_rowid();

    let todo = get_by_id(&tx, id)?;

    tx.commit()?;

    Ok(todo)
}

/// Gets todos.
pub fn get(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare(
        "
            SELECT t.id, t.todo, t.info, t.status, p.name as project, t.due_date
            FROM todos t LEFT OUTER JOIN projects p ON t.project_id = p.id
        ",
    )?;
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get("id")?,
                todo: row.get("todo")?,
                info: row.get("info")?,
                status: row.get("status")?,
                project: row.get("project")?,
                due_date: row.get("due_date")?,
                created_at: row.get("created_at")?,
                completed_at: row.get("completed_at")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(todos)
}

/// Gets todo by ID.
pub fn get_by_id(conn: &Connection, todo_id: i64) -> Result<Todo> {
    let mut stmt = conn.prepare(
        "
        SELECT t.id, t.todo, t.info, t.status, p.name AS project, t.due_date, t.created_at, t.completed_at
        FROM todos t LEFT OUTER JOIN projects p ON t.project_id = p.id
        WHERE t.id = ?1",
    )?;

    Ok(stmt.query_row([todo_id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo: row.get(1)?,
            info: row.get(2)?,
            status: row.get(3)?,
            project: row.get(4)?,
            due_date: row.get(5)?,
            created_at: row.get(6)?,
            completed_at: row.get(7)?,
        })
    })?)
}

/// Updates an existing todo.
pub fn update(conn: &mut Connection, todo: TodoRecord) -> Result<Todo> {
    tracing::debug!("updating todo");
    let tx = conn.transaction()?;

    let current_todo = get_by_id(&tx, todo.id)?;
    let completed_at = match (current_todo.status, todo.status) {
        (Status::ToDo, Status::Done) => Some(Local::now().date_naive()),
        (Status::InProgress, Status::Done) => Some(Local::now().date_naive()),
        (Status::Done, Status::InProgress) => None,
        (Status::Done, Status::ToDo) => None,
        _ => current_todo.completed_at,
    };

    tx.execute(
        "
        UPDATE todos
        SET todo = ?2, info = ?3, status = ?4, project_id = ?5, due_date = ?6, completed_at = ?7
        WHERE id = ?1
        ",
        params![
            todo.id,
            todo.todo,
            todo.info,
            todo.status,
            todo.project_id,
            todo.due_date,
            completed_at
        ],
    )?;

    let updated_todo = get_by_id(&tx, todo.id)?;

    tx.commit()?;

    Ok(updated_todo)
}

pub fn update_status(conn: &Connection, todo_id: i64, status: Status) -> Result<()> {
    tracing::debug!("updating todo status");
    conn.execute(
        "UPDATE todos SET status = ?2 WHERE id = ?1",
        params![todo_id, status],
    )?;
    Ok(())
}

/// Deletes a todo by ID.
pub fn delete(conn: &Connection, todo_id: i64) -> Result<()> {
    let mut stmt = conn.prepare(
        "
        DELETE
        FROM todos
        WHERE id = ?1",
    )?;

    stmt.execute([todo_id])?;

    Ok(())
}
