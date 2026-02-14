pub mod core;
mod tray_icon;
mod use_cases;

use std::sync::Arc;

use crate::core::StateSummaryGateway;
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
    let state_summary_gateway = Arc::new(StateSummaryGateway::new());
    let service = MonitoringService::new(state_summary_gateway.clone());
    service.create_counter();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tray_icon::init_with(state_summary_gateway.clone()))
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
