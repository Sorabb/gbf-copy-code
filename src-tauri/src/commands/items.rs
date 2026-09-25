use crate::{
    models::{AddCodesResult, Item},
    services::item_service,
    state::AppState,
};
use tauri::State;

#[tauri::command]
pub fn get_items(state: State<'_, AppState>, list_id: i64) -> Result<Vec<Item>, String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    item_service::get_active_items(&connection, list_id)
}

#[tauri::command]
pub fn add_codes(
    state: State<'_, AppState>,
    list_id: i64,
    codes: Vec<String>,
) -> Result<AddCodesResult, String> {
    let mut connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    item_service::add_codes(&mut connection, list_id, codes)
}

#[tauri::command]
pub fn delete_code(state: State<'_, AppState>, list_id: i64, code: String) -> Result<(), String> {
    let connection = state
        .database
        .lock()
        .map_err(|_| "数据库锁已损坏".to_string())?;
    item_service::delete_code(&connection, list_id, &code)
}
