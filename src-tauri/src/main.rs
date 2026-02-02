// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod utils;

use commands::{obsidian, proton, system};
use services::system_monitor::SystemMonitor;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

/// Application state shared across commands
pub struct AppState {
    system_monitor: Arc<Mutex<SystemMonitor>>,
}

fn main() {
    // Initialize system monitor
    let system_monitor = Arc::new(Mutex::new(
        SystemMonitor::new().expect("Failed to initialize system monitor"),
    ));

    // Generate TypeScript bindings
    #[cfg(debug_assertions)]
    {
        tauri_specta::Builder::<tauri::Wry>::new()
            .commands(tauri_specta::collect_commands![
                system::get_system_stats,
                system::start_system_monitoring,
                system::stop_system_monitoring,
                obsidian::list_notes,
                obsidian::read_note,
                obsidian::save_note,
                proton::list_proton_versions,
            ])
            .events(tauri_specta::collect_events![
                models::system::SystemStatsEvent,
            ])
            .path("../src/lib/tauri.ts")
            .build()
            .expect("Failed to generate TypeScript bindings");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { system_monitor })
        .invoke_handler(tauri::generate_handler![
            system::get_system_stats,
            system::start_system_monitoring,
            system::stop_system_monitoring,
            obsidian::list_notes,
            obsidian::read_note,
            obsidian::save_note,
            proton::list_proton_versions,
        ])
        .setup(|app| {
            // Clone for background task
            let app_handle = app.handle().clone();
            let monitor = app.state::<AppState>().system_monitor.clone();

            // Spawn background telemetry task
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    
                    let mut monitor = monitor.lock().await;
                    match monitor.snapshot() {
                        Ok(stats) => {
                            let _ = app_handle.emit("system-stats", stats);
                        }
                        Err(e) => {
                            eprintln!("System monitoring error: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
