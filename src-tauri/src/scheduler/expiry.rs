use crate::{services::item_service, state::AppState};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let result = {
                let state = app.state::<AppState>();
                let expired = match state.database.lock() {
                    Ok(mut connection) => item_service::expire_items(&mut connection),
                    Err(_) => Err("数据库锁已损坏".into()),
                };
                expired
            };
            match result {
                Ok(expired) if !expired.is_empty() => {
                    if let Err(error) = app.emit("items-expired", expired) {
                        tracing::error!(%error, "failed to emit expiry event");
                    }
                }
                Ok(_) => {}
                Err(error) => tracing::error!(%error, "expiry worker failed"),
            }
        }
    });
}
