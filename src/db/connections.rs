use super::migrations::migrate;
use rusqlite::{Connection, Result};

/// Connects to the sqlite database
/// Creates the database if not present
pub fn create_database() -> Result<Connection> {
    // Connect to db (create database if it doesn't exist)
    let mut conn = Connection::open("todo.db")?;
    migrate(&mut conn)?;
    Ok(conn)
}
