use crate::models::system::SystemStats;
use crate::AppState;
use tauri::State;

/// Get current system statistics (one-time snapshot)
#[tauri::command]
#[specta::specta]
pub async fn get_system_stats(state: State<'_, AppState>) -> Result<SystemStats, String> {
    let mut monitor = state.system_monitor.lock().await;
    monitor.snapshot().map_err(|e| e.to_string())
}

/// Start continuous system monitoring (emits events)
#[tauri::command]
#[specta::specta]
pub async fn start_system_monitoring() -> Result<(), String> {
    // Monitoring is already running in background task (see main.rs setup)
    Ok(())
}

/// Stop system monitoring
#[tauri::command]
#[specta::specta]
pub async fn stop_system_monitoring() -> Result<(), String> {
    // TODO: Implement proper shutdown mechanism if needed
    Ok(())
}
