use crate::models::todo::{NewTodoRecord, Todo, TodoRecord};
use rusqlite::{Connection, Result, params};

/// Creates a new todo and returns the generated ID.
pub fn create(conn: &Connection, todo: NewTodoRecord) -> Result<i64> {
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
pub fn get_all(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare(
        "
            SELECT t.id, t.todo, t.info, t.status, p.project, t.due_date
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
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(todos)
}

/// Gets todo by ID.
pub fn get_by_id(conn: &Connection, todo_id: &i64) -> Result<Todo> {
    let mut stmt = conn.prepare(
        "
        SELECT t.id, t.todo, t.info, t.status, p.name as project, t.due_date
        FROM todos t LEFT OUTER JOIN projects p ON t.project_id = p.id
        WHERE id = ?1",
    )?;

    Ok(stmt.query_row([todo_id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo: row.get(1)?,
            info: row.get(2)?,
            status: row.get(3)?,
            project: row.get(4)?,
            due_date: row.get(5)?,
        })
    })?)
}

/// Updates an existing todo.
pub fn update(conn: &Connection, todo: TodoRecord) -> Result<()> {
    let mut stmt = conn.prepare(
        "
        UPDATE todos
        SET todo = ?2, info = ?3, status = ?4, project_id = ?5, due_date = ?6
        WHERE id = ?1
        ",
    )?;

    stmt.execute(params![
        todo.id,
        todo.todo,
        todo.info,
        todo.status,
        todo.project_id,
        todo.due_date,
    ])?;

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
