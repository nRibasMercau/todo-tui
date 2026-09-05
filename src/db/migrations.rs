use rusqlite::{Connection, Result};

pub fn migrate(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    let version: i32 = tx.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if version < 1 {
        migration_1(&tx)?;
    }

    if version < 2 {
        migration_2(&tx)?;
    }

    tx.commit()?;

    Ok(())
}

fn migration_1(tx: &rusqlite::Transaction<'_>) -> Result<()> {
    tx.execute_batch(include_str!("./001_initial_tables.sql"))?;
    tx.execute_batch("PRAGMA user_version = 1")?;
    Ok(())
}

fn migration_2(tx: &rusqlite::Transaction<'_>) -> Result<()> {
    tx.execute_batch(include_str!("./002_add_project_archived.sql"))?;
    tx.execute_batch("PRAGMA user_version = 2")?;
    Ok(())
}
