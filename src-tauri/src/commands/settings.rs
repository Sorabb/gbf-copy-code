use crate::{models::AppSettings, services::settings_service, state::AppState};
use tauri::State;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    settings_service::get(&connection, &state.database_path)
}

#[tauri::command]
pub fn complete_first_run(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    settings_service::complete_first_run(&connection, &state.database_path)
}

#[tauri::command]
pub fn set_last_selected_list(state: State<'_, AppState>, list_id: i64) -> Result<(), String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    settings_service::set_last_selected_list(&connection, list_id)
}
