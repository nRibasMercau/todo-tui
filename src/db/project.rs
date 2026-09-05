use crate::models::project::NewProject;
use rusqlite::{Connection, Result};

/// Creates a new project and returns the generated ID.
pub fn create(conn: &Connection, project: &NewProject) -> Result<i64> {
    conn.execute(
        "
        INSERT INTO projects (name, archived)
        VALUES (?1, ?2)
        ",
        (&project.name, false),
    )?;
    Ok(conn.last_insert_rowid())
}

/// Gets project ID by name
pub fn get_by_name(conn: &Connection, project_name: &str) -> Result<Option<i64>> {
    conn.query_row(
        "SELECT id FROM projects WHERE name = ?1",
        [project_name],
        |row| row.get("id"),
    )
}
