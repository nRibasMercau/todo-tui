use rusqlite::{Connection, Result};

/// Connects to the sqlite database
/// Creates the database if not present
pub fn create_database() -> Result<Connection> {
    // Connect to db (create database if it doesn't exist)
    let conn = Connection::open("todo.db")?;

    // Create main table with todos
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                todo TEXT NOT NULL,
                info TEXT,
                status TEXT NOT NULL DEFAULT 'todo',
                project_id INTEGER,
                due_date DATE,
                FOREIGN KEY ( project_id ) REFERENCES projects( id )
            )",
        [],
    )?;

    // Create projects table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            ",
        [],
    )?;

    Ok(conn)
}
