mod commands;
mod db;
mod models;
mod networking;
mod scheduler;
mod services;
mod state;

use crate::{services::item_service, state::AppState};
use std::{fs, io};
use tauri::Manager;
use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .try_init();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            tracing::info!("application starting");
            let data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&data_dir)?;
            let database_path = data_dir.join("local-code-lists.db");
            let mut connection = db::open(&database_path).map_err(io::Error::other)?;
            let removed = item_service::expire_items(&mut connection)
                .map_err(io::Error::other)?
                .len();
            tracing::info!(path = %database_path.display(), removed, "database initialized");
            app.manage(AppState::new(connection, database_path));
            scheduler::expiry::spawn(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::complete_first_run,
            commands::settings::set_last_selected_list,
            commands::lists::get_lists,
            commands::lists::create_list,
            commands::lists::rename_list,
            commands::lists::delete_list,
            commands::items::get_items,
            commands::items::add_codes,
            commands::items::delete_code,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
