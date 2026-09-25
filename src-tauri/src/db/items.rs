use crate::models::{ExpiredItem, Item};
use rusqlite::{params, Connection, Result};

pub fn active_for_list(connection: &Connection, list_id: i64, now: i64) -> Result<Vec<Item>> {
    let mut statement = connection.prepare(
        "SELECT list_id, code, created_at, expires_at FROM items
         WHERE list_id = ?1 AND expires_at > ?2 ORDER BY created_at ASC, rowid ASC",
    )?;
    let rows = statement
        .query_map(params![list_id, now], |row| {
            Ok(Item {
                list_id: row.get(0)?,
                code: row.get(1)?,
                created_at: row.get(2)?,
                expires_at: row.get(3)?,
            })
        })?
        .collect();
    rows
}

pub fn contains(connection: &Connection, list_id: i64, code: &str, now: i64) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM items WHERE list_id = ?1 AND code = ?2 AND expires_at > ?3)",
        params![list_id, code, now],
        |row| row.get(0),
    )
}

pub fn insert(connection: &Connection, item: &Item) -> Result<usize> {
    connection.execute(
        "INSERT INTO items(list_id, code, created_at, expires_at) VALUES (?1, ?2, ?3, ?4)",
        params![item.list_id, item.code, item.created_at, item.expires_at],
    )
}

pub fn delete(connection: &Connection, list_id: i64, code: &str) -> Result<usize> {
    connection.execute(
        "DELETE FROM items WHERE list_id = ?1 AND code = ?2",
        params![list_id, code],
    )
}

pub fn expired(connection: &Connection, now: i64) -> Result<Vec<ExpiredItem>> {
    let mut statement =
        connection.prepare("SELECT list_id, code FROM items WHERE expires_at <= ?1")?;
    let rows = statement
        .query_map([now], |row| {
            Ok(ExpiredItem {
                list_id: row.get(0)?,
                code: row.get(1)?,
            })
        })?
        .collect();
    rows
}

pub fn delete_expired(connection: &Connection, now: i64) -> Result<usize> {
    connection.execute("DELETE FROM items WHERE expires_at <= ?1", [now])
}
