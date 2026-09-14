use crate::models::project::{NewProject, Project};
use rusqlite::{Connection, Result, params};

/// Creates a new project and returns the generated project.
pub fn create(conn: &mut Connection, project: NewProject) -> Result<Project> {
    tracing::debug!("creating project");
    let tx = conn.transaction()?;
    tx.execute(
        "
        INSERT INTO projects (name, archived)
        VALUES (?1, ?2)
        ",
        (&project.name, false),
    )?;

    let project_id = tx.last_insert_rowid();
    let project = get_by_id(&tx, project_id)?;
    tx.commit()?;

    Ok(project)
}

/// Gets project ID by name
pub fn get_by_name(conn: &Connection, project_name: &str) -> Result<Option<i64>> {
    match conn.query_row(
        "SELECT id FROM projects WHERE name = ?1",
        [project_name],
        |row| row.get("id"),
    ) {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

/// Gets project by iD
pub fn get_by_id(conn: &Connection, project_id: i64) -> Result<Project> {
    tracing::debug!("getting project");
    let mut stmt = conn.prepare(
        "
        SELECT id, name, archived, created_at
        FROM projects 
        WHERE id = ?1",
    )?;

    Ok(stmt.query_row([project_id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            archived: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?)
}

/// Gets projects.
pub fn get(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "
        SELECT id, name, archived, created_at
        FROM projects
        ",
    )?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get("id")?,
                name: row.get("name")?,
                archived: row.get("archived")?,
                created_at: row.get("created_at")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

/// Updates a project and returns the updated project.
pub fn update(conn: &mut Connection, project: Project) -> Result<Project> {
    let tx = conn.transaction()?;

    tx.execute(
        "
            UPDATE projects
            SET name = ?2, archived = ?3
            WHERE id = ?1
        ",
        params![project.id, project.name, project.archived],
    )?;

    let updated_project = get_by_id(&tx, project.id)?;

    tx.commit()?;

    Ok(updated_project)
}

/// Deletes a project.
pub fn delete(conn: &mut Connection, project_id: i64) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;

    tx.commit()?;

    Ok(())
}
