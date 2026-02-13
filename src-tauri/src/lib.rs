pub mod core;
mod tray_icon;
mod use_cases;

use std::sync::{Arc, Mutex};

use crate::core::StateSummaryDispatcher;
use crate::use_cases::counter::MonitoringService;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default)]
struct AppState {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let dispatcher = Arc::new(Mutex::new(StateSummaryDispatcher::new()));
    let setup_dispatcher = dispatcher.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tray_icon::init_with(dispatcher.clone()))
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |_app| {
            let service = MonitoringService::new(setup_dispatcher.clone());
            service.create_counter();
            // Spawn background thread that runs every 2 seconds
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
