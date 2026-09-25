use crate::{models::ListInfo, services::list_service, state::AppState};
use tauri::State;

#[tauri::command]
pub fn get_lists(state: State<'_, AppState>) -> Result<Vec<ListInfo>, String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    list_service::get_or_create_lists(&connection)
}

#[tauri::command]
pub fn create_list(state: State<'_, AppState>, name: String) -> Result<ListInfo, String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    list_service::create(&connection, &name)
}

#[tauri::command]
pub fn rename_list(state: State<'_, AppState>, id: i64, name: String) -> Result<(), String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    list_service::rename(&connection, id, &name)
}

#[tauri::command]
pub fn delete_list(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    list_service::delete(&connection, id)
}
