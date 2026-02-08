mod use_cases;

use crate::use_cases::counter::{CounterData, MonitoringService, StatusObserver};
use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

impl StatusObserver for TauriFrontend {
    fn on_update(&self, data: CounterData) {
        if let Err(e) = self.handle.emit("background-update", data) {
            eprintln!("Failed to emit event: {}", e);
        }
    }
}

#[derive(Default)]
struct AppState {}

struct TauriFrontend {
    handle: AppHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let _app_handle = app.handle().clone();
            let service = MonitoringService::new(Box::new(TauriFrontend {
                handle: app.handle().clone(),
            }));
            service.create_counter();
            // Spawn background thread that runs every 2 seconds
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
