use crate::models::ListInfo;
use rusqlite::{params, Connection, Result};

pub fn all(connection: &Connection) -> Result<Vec<ListInfo>> {
    let mut statement = connection
        .prepare("SELECT id, name, created_at FROM lists ORDER BY created_at ASC, id ASC")?;
    let rows = statement
        .query_map([], |row| {
            Ok(ListInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?
        .collect();
    rows
}

pub fn insert(connection: &Connection, name: &str, now: i64) -> Result<ListInfo> {
    connection.execute(
        "INSERT INTO lists(name, created_at) VALUES (?1, ?2)",
        params![name, now],
    )?;
    Ok(ListInfo {
        id: connection.last_insert_rowid(),
        name: name.to_owned(),
        created_at: now,
    })
}

pub fn rename(connection: &Connection, id: i64, name: &str) -> Result<usize> {
    connection.execute(
        "UPDATE lists SET name = ?1 WHERE id = ?2",
        params![name, id],
    )
}

pub fn delete(connection: &Connection, id: i64) -> Result<usize> {
    connection.execute("DELETE FROM lists WHERE id = ?1", [id])
}

pub fn exists(connection: &Connection, id: i64) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM lists WHERE id = ?1)",
        [id],
        |row| row.get(0),
    )
}
