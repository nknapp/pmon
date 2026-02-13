pub mod core;
mod taskbar_notifications;
mod use_cases;

use crate::core::NotificationStateDispatcher;
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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            taskbar_notifications::setup_tray(app)?;
            let mut dispatcher = NotificationStateDispatcher::new();
            dispatcher.add_controller(taskbar_notifications::create_controller(
                app.handle().clone(),
            ));
            let service = MonitoringService::new(dispatcher);
            service.create_counter();
            // Spawn background thread that runs every 2 seconds
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
