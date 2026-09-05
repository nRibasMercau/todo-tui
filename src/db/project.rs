use crate::models::project::{NewProject, Project};
use rusqlite::{Connection, Result, params};

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

/// Gets projects.
pub fn get_all(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "
        SELECT id, name, archived
        FROM projects
        ",
    )?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get("id")?,
                name: row.get("name")?,
                archived: row.get("archived")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

/// Updates a project.
pub fn update(conn: &Connection, project: Project) -> Result<()> {
    conn.execute(
        "
            UPDATE projects
            SET name = ?2, archived = ?3
            WHERE id = ?1
        ",
        params![project.id, project.name, project.archived],
    )?;

    Ok(())
}

/// Deletes a project.
pub fn delete(conn: &Connection, project_id: i64) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;

    Ok(())
}
