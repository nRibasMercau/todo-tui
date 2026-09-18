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
pub fn delete(conn: &Connection, project_id: i64) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;

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
    fn creates_project() -> Result<()> {
        let mut conn = test_db()?;

        let new_project = NewProject {
            name: String::from("My project"),
        };

        let project = create(&mut conn, new_project)?;

        assert_eq!(project.name, "My project");

        let projects = get(&conn)?;

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project.id);
        assert_eq!(projects[0].name, project.name);
        assert_eq!(projects[0].archived, project.archived);

        Ok(())
    }

    #[test]
    fn deletes_project() -> Result<()> {
        let mut conn = test_db()?;

        let new_project = NewProject {
            name: String::from("My project"),
        };

        let project = create(&mut conn, new_project)?;

        delete(&mut conn, project.id)?;

        let projects = get(&mut conn)?;

        assert!(projects.is_empty());

        Ok(())
    }

    #[test]
    fn gets_projects() -> Result<()> {
        let mut conn = test_db()?;

        let project_1 = NewProject {
            name: String::from("My project 1"),
        };

        let project_2 = NewProject {
            name: String::from("My project 2"),
        };

        create(&mut conn, project_1)?;
        create(&mut conn, project_2)?;

        let projects = get(&conn)?;

        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "My project 1");
        assert_eq!(projects[1].name, "My project 2");
        assert_eq!(projects[0].archived, false);
        assert_eq!(projects[1].archived, false);

        Ok(())
    }

    #[test]
    fn gets_project_by_id() -> Result<()> {
        let mut conn = test_db()?;

        let project_1 = NewProject {
            name: String::from("My project 1"),
        };

        let project_2 = NewProject {
            name: String::from("My project 2"),
        };

        let project_1 = create(&mut conn, project_1)?;
        let project_2 = create(&mut conn, project_2)?;

        let project_1_result = get_by_id(&conn, project_1.id)?;
        let project_2_result = get_by_id(&conn, project_2.id)?;

        assert_eq!(project_1_result.id, project_1.id);
        assert_eq!(project_1_result.name, project_1.name);
        assert_eq!(project_1_result.archived, project_1.archived);

        assert_eq!(project_2_result.id, project_2.id);
        assert_eq!(project_2_result.name, project_2.name);
        assert_eq!(project_2_result.archived, project_2.archived);

        Ok(())
    }

    #[test]
    fn gets_project_by_name() -> Result<()> {
        let mut conn = test_db()?;

        let new_project = NewProject {
            name: String::from("My project"),
        };

        let project = create(&mut conn, new_project)?;

        let existing_project_id = get_by_name(&mut conn, &project.name)?;
        let nonexisting_project_id = get_by_name(&mut conn, "Inexistent project")?;

        assert!(existing_project_id.is_some());
        assert_eq!(existing_project_id, Some(project.id));
        assert!(!nonexisting_project_id.is_some());

        Ok(())
    }

    #[test]
    fn updates_project() -> Result<()> {
        let mut conn = test_db()?;

        let new_project = NewProject {
            name: String::from("My project"),
        };

        let project = create(&mut conn, new_project)?;

        let updated_project = Project {
            id: project.id,
            name: String::from("My project UPDATED"),
            archived: false,
            created_at: project.created_at,
        };

        update(&mut conn, updated_project)?;

        let project = get_by_id(&conn, project.id)?;

        assert_eq!(project.name, "My project UPDATED");

        let updated_project = Project {
            id: project.id,
            name: String::from("My project UPDATED"),
            archived: true,
            created_at: project.created_at,
        };

        update(&mut conn, updated_project)?;

        assert_eq!(project.archived, false);

        Ok(())
    }
}
