use crate::models::todo::{NewTodoRecord, Status, Todo, TodoRecord};
use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};

/// Creates a new todo and returns the new todo.
pub fn create(conn: &mut Connection, todo: &NewTodoRecord) -> Result<Todo> {
    let status = todo.status.as_str();

    tracing::debug!("inserting todo");
    let tx = conn.transaction()?;
    tx.execute(
        "
        INSERT INTO todos (todo, info, status, project_id, due_date, completed_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &todo.todo,
            &todo.info,
            status,
            &todo.project_id,
            &todo.due_date,
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
pub fn update(conn: &mut Connection, todo: &TodoRecord) -> Result<Todo> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::migrate;

    fn test_db() -> Result<Connection> {
        let mut conn = Connection::open_in_memory()?;
        migrate(&mut conn)?;
        Ok(conn)
    }

    #[test]
    fn creates_todo() -> Result<()> {
        let mut conn = test_db()?;

        let new_todo = NewTodoRecord {
            todo: String::from("My todo"),
            status: Status::ToDo,
            info: String::from("This is a test todo"),
            project_id: None,
            due_date: None,
            completed_at: None,
        };

        let todo = create(&mut conn, &new_todo)?;

        assert_eq!(todo.todo, "My todo");

        let todos = get(&conn)?;

        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, todo.id);
        assert_eq!(todos[0].todo, todo.todo);
        assert_eq!(todos[0].status, todo.status);
        assert_eq!(todos[0].project_id, todo.project_id);
        assert_eq!(todos[0].due_date, todo.due_date);
        assert_eq!(todos[0].completed_at, todo.completed_at);
        assert_eq!(todos[0].created_at, todo.created_at);

        Ok(())
    }

    #[test]
    fn deletes_todo() -> Result<()> {
        let mut conn = test_db()?;

        let new_todo = NewTodoRecord {
            todo: String::from("My todo"),
            status: Status::ToDo,
            info: String::from("This is a test todo"),
            project_id: None,
            due_date: None,
            completed_at: None,
        };

        let todo = create(&mut conn, &new_todo)?;
        let todos = get(&conn)?;
        assert_eq!(todos.len(), 1);

        delete(&mut conn, todo.id)?;
        let todos = get(&conn)?;
        assert!(todos.is_empty());

        Ok(())
    }

    #[test]
    fn gets_todos() -> Result<()> {
        let mut conn = test_db()?;

        let todo_1 = NewTodoRecord {
            todo: String::from("My todo 1"),
            status: Status::ToDo,
            info: String::from("This is a test todo 1"),
            project_id: None,
            due_date: NaiveDate::from_ymd_opt(2026, 12, 31),
            completed_at: None,
        };

        let todo_2 = NewTodoRecord {
            todo: String::from("My todo 2"),
            status: Status::ToDo,
            info: String::from("This is a test todo 2"),
            project_id: None,
            due_date: None,
            completed_at: None,
        };

        create(&mut conn, &todo_1)?;
        create(&mut conn, &todo_2)?;

        let todos = get(&conn)?;

        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].todo, todo_1.todo);
        assert_eq!(todos[0].status, todo_1.status);
        assert_eq!(todos[0].info, todo_1.info);
        assert_eq!(todos[0].project_id, todo_1.project_id);
        assert_eq!(todos[0].due_date, todo_1.due_date);
        assert_eq!(todos[0].completed_at, todo_1.completed_at);
        assert_eq!(todos[1].todo, todo_2.todo);
        assert_eq!(todos[1].status, todo_2.status);
        assert_eq!(todos[1].info, todo_2.info);
        assert_eq!(todos[1].project_id, todo_2.project_id);
        assert_eq!(todos[1].due_date, todo_2.due_date);
        assert_eq!(todos[1].completed_at, todo_2.completed_at);

        Ok(())
    }

    #[test]
    fn gets_todo_by_id() -> Result<()> {
        let mut conn = test_db()?;

        let todo = NewTodoRecord {
            todo: String::from("My todo 1"),
            status: Status::ToDo,
            info: String::from("This is a test todo 1"),
            project_id: None,
            due_date: NaiveDate::from_ymd_opt(2026, 12, 31),
            completed_at: None,
        };

        let todo = create(&mut conn, &todo)?;

        let todo_1_result = get_by_id(&conn, todo.id)?;

        assert_eq!(todo_1_result.id, todo.id);
        assert_eq!(todo_1_result.status, todo.status);
        assert_eq!(todo_1_result.info, todo.info);
        assert_eq!(todo_1_result.project_id, todo.project_id);
        assert_eq!(todo_1_result.due_date, todo.due_date);
        assert_eq!(todo_1_result.completed_at, todo.completed_at);
        assert_eq!(todo_1_result.created_at, todo.created_at);

        Ok(())
    }

    #[test]
    fn updates_todo() -> Result<()> {
        let mut conn = test_db()?;

        let todo = NewTodoRecord {
            todo: String::from("My todo 1"),
            status: Status::ToDo,
            info: String::from("This is a test todo 1"),
            project_id: None,
            due_date: NaiveDate::from_ymd_opt(2026, 12, 31),
            completed_at: None,
        };

        let todo = create(&mut conn, &todo)?;

        let updated_todo = TodoRecord {
            id: todo.id,
            todo: String::from("My todo 1 UPDATED"),
            info: todo.info.clone(),
            status: todo.status,
            project_id: todo.project_id,
            due_date: todo.due_date,
            created_at: todo.created_at,
            completed_at: todo.completed_at,
        };

        let updated_todo = update(&mut conn, &updated_todo)?;

        assert_ne!(updated_todo.todo, todo.todo);
        assert_eq!(updated_todo.todo, "My todo 1 UPDATED");
        assert_eq!(updated_todo.info, todo.info);
        assert_eq!(updated_todo.status, todo.status);
        assert_eq!(updated_todo.project_id, todo.project_id);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_eq!(updated_todo.completed_at, todo.completed_at);
        assert_eq!(updated_todo.due_date, todo.due_date);

        let updated_todo = TodoRecord {
            id: todo.id,
            todo: String::from("My todo 1 UPDATED"),
            info: todo.info.clone(),
            status: todo.status,
            project_id: todo.project_id,
            due_date: NaiveDate::from_ymd_opt(2027, 9, 1),
            created_at: todo.created_at,
            completed_at: todo.completed_at,
        };

        let updated_todo = update(&mut conn, &updated_todo)?;

        assert_ne!(updated_todo.todo, todo.todo);
        assert_eq!(updated_todo.todo, "My todo 1 UPDATED");
        assert_eq!(updated_todo.info, todo.info);
        assert_eq!(updated_todo.status, todo.status);
        assert_eq!(updated_todo.project_id, todo.project_id);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_eq!(updated_todo.completed_at, todo.completed_at);
        assert_ne!(updated_todo.due_date, todo.due_date);
        assert_eq!(updated_todo.due_date, NaiveDate::from_ymd_opt(2027, 9, 1));

        Ok(())
    }

    #[test]
    fn updates_todo_status() -> Result<()> {
        let mut conn = test_db()?;

        let todo = NewTodoRecord {
            todo: String::from("My todo 1"),
            status: Status::ToDo,
            info: String::from("This is a test todo 1"),
            project_id: None,
            due_date: NaiveDate::from_ymd_opt(2026, 12, 31),
            completed_at: None,
        };

        let todo = create(&mut conn, &todo)?;

        update_status(&conn, todo.id, Status::InProgress, None)?;

        let updated_todo = get_by_id(&conn, todo.id)?;

        assert_ne!(updated_todo.status, todo.status);
        assert_eq!(updated_todo.status, Status::InProgress);
        assert_eq!(updated_todo.todo, todo.todo);
        assert_eq!(updated_todo.info, todo.info);
        assert_eq!(updated_todo.project_id, todo.project_id);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_eq!(updated_todo.due_date, todo.due_date);
        assert_eq!(updated_todo.completed_at, todo.completed_at);

        update_status(
            &conn,
            todo.id,
            Status::Done,
            NaiveDate::from_ymd_opt(2026, 9, 18),
        )?;

        let updated_todo = get_by_id(&conn, todo.id)?;

        assert_ne!(updated_todo.status, todo.status);
        assert_eq!(updated_todo.status, Status::Done);
        assert_eq!(updated_todo.todo, todo.todo);
        assert_eq!(updated_todo.info, todo.info);
        assert_eq!(updated_todo.project_id, todo.project_id);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_ne!(updated_todo.completed_at, todo.completed_at);
        assert_eq!(
            updated_todo.completed_at,
            NaiveDate::from_ymd_opt(2026, 9, 18)
        );
        assert_eq!(updated_todo.due_date, todo.due_date);

        Ok(())
    }
}
