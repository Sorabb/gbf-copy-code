use crate::{db, models::AppSettings};
use rusqlite::Connection;
use std::path::Path;

pub fn get(connection: &Connection, database_path: &Path) -> Result<AppSettings, String> {
    let first_run_completed = db::settings::get(connection, "first_run_completed")
        .map_err(|error| error.to_string())?
        .as_deref()
        == Some("true");
    let run_mode = db::settings::get(connection, "run_mode")
        .map_err(|error| error.to_string())?
        .unwrap_or_else(|| "local".into());
    let last_selected_list_id = db::settings::get(connection, "last_selected_list_id")
        .map_err(|error| error.to_string())?
        .and_then(|value| value.parse().ok());
    Ok(AppSettings {
        first_run_completed,
        run_mode,
        last_selected_list_id,
        database_path: database_path.to_string_lossy().into_owned(),
    })
}

pub fn complete_first_run(
    connection: &Connection,
    database_path: &Path,
) -> Result<AppSettings, String> {
    db::settings::set(connection, "first_run_completed", "true")
        .map_err(|error| error.to_string())?;
    db::settings::set(connection, "run_mode", "local").map_err(|error| error.to_string())?;
    get(connection, database_path)
}

pub fn set_last_selected_list(connection: &Connection, list_id: i64) -> Result<(), String> {
    if !db::lists::exists(connection, list_id).map_err(|error| error.to_string())? {
        return Err("列表不存在".into());
    }
    db::settings::set(connection, "last_selected_list_id", &list_id.to_string())
        .map_err(|error| error.to_string())
}
