use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub first_run_completed: bool,
    pub run_mode: String,
    pub last_selected_list_id: Option<i64>,
    pub database_path: String,
}
