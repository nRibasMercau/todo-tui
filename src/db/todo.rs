use crate::models::todo::{NewTodoRecord, Status, Todo, TodoRecord};
use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};

/// Creates a new todo and returns the new todo.
pub fn create(conn: &mut Connection, todo: NewTodoRecord) -> Result<Todo> {
    let status = todo.status.as_str();

    tracing::debug!("inserting todo");
    let tx = conn.transaction()?;
    tx.execute(
        "
        INSERT INTO todos (todo, info, status, project_id, due_date, created_at, completed_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &todo.todo,
            &todo.info,
            status,
            &todo.project_id,
            &todo.due_date,
            &todo.created_at,
            &todo.completed_at,
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
            SELECT t.id, t.todo, t.info, t.status, t.project_id, p.name as project, t.due_date, t.created_at, t.completed_at
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
                project_id: row.get("project_id")?,
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
        SELECT t.id, t.todo, t.info, t.status, t.project_id, p.name AS project, t.due_date, t.created_at, t.completed_at
        FROM todos t LEFT OUTER JOIN projects p ON t.project_id = p.id
        WHERE t.id = ?1",
    )?;

    Ok(stmt.query_row([todo_id], |row| {
        Ok(Todo {
            id: row.get("id")?,
            todo: row.get("todo")?,
            info: row.get("info")?,
            status: row.get("status")?,
            project_id: row.get("project_id")?,
            project: row.get("project")?,
            due_date: row.get("due_date")?,
            created_at: row.get("created_at")?,
            completed_at: row.get("completed_at")?,
        })
    })?)
}

/// Updates an existing todo.
pub fn update(conn: &mut Connection, todo: TodoRecord) -> Result<Todo> {
    tracing::debug!("updating todo");
    let tx = conn.transaction()?;

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
            todo.completed_at
        ],
    )?;

    let updated_todo = get_by_id(&tx, todo.id)?;

    tx.commit()?;

    Ok(updated_todo)
}

pub fn update_status(
    conn: &Connection,
    todo_id: i64,
    status: Status,
    completed_at: Option<NaiveDate>,
) -> Result<()> {
    tracing::debug!("updating todo status");
    conn.execute(
        "UPDATE todos SET status = ?2, completed_at = ?3 WHERE id = ?1",
        params![todo_id, status, completed_at],
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
