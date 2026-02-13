pub mod core;
mod tray_icon;
mod use_cases;

use std::sync::{Arc, Mutex};

use crate::core::StateSummaryDispatcher;
use crate::use_cases::counter::MonitoringService;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default)]
struct AppState {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tray_icon::init())
        .manage(Arc::new(Mutex::new(StateSummaryDispatcher::new())))
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let dispatcher = app
                .state::<Arc<Mutex<StateSummaryDispatcher>>>()
                .inner()
                .clone();
            let service = MonitoringService::new(dispatcher);
            service.create_counter();
            // Spawn background thread that runs every 2 seconds
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
