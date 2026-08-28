use rusqlite::{Connection, Result};

/// Connects to the sqlite database
/// Creates the database if not present
pub fn create_database() -> Result<Connection> {
    // Connect to db (create database if it doesn't exist)
    let conn = Connection::open("todo.db")?;

    Ok(conn)
}
