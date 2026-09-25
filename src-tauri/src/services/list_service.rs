use crate::{db, models::ListInfo, services::now_millis};
use rusqlite::Connection;

pub fn get_or_create_lists(connection: &Connection) -> Result<Vec<ListInfo>, String> {
    let mut lists = db::lists::all(connection).map_err(|error| error.to_string())?;
    if lists.is_empty() {
        let created = db::lists::insert(connection, "列表 1", now_millis())
            .map_err(|error| error.to_string())?;
        tracing::info!(list_id = created.id, "created default list");
        lists.push(created);
    }
    Ok(lists)
}

pub fn create(connection: &Connection, name: &str) -> Result<ListInfo, String> {
    let name = validated_name(name)?;
    let list =
        db::lists::insert(connection, name, now_millis()).map_err(|error| error.to_string())?;
    tracing::info!(list_id = list.id, "created list");
    Ok(list)
}

pub fn rename(connection: &Connection, id: i64, name: &str) -> Result<(), String> {
    let name = validated_name(name)?;
    if db::lists::rename(connection, id, name).map_err(|error| error.to_string())? == 0 {
        return Err("列表不存在".into());
    }
    tracing::info!(list_id = id, "renamed list");
    Ok(())
}

pub fn delete(connection: &Connection, id: i64) -> Result<(), String> {
    if db::lists::delete(connection, id).map_err(|error| error.to_string())? == 0 {
        return Err("列表不存在".into());
    }
    tracing::info!(list_id = id, "deleted list");
    Ok(())
}

fn validated_name(name: &str) -> Result<&str, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("列表名称不能为空".into());
    }
    if name.chars().count() > 80 {
        return Err("列表名称不能超过 80 个字符".into());
    }
    Ok(name)
}
