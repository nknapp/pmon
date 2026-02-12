pub mod core;
mod taskbar_notifications;
mod use_cases;

use crate::use_cases::counter::{CounterData, MonitoringService, StatusObserver};
use tauri::{image::Image, menu::Menu, tray::TrayIconBuilder, AppHandle, Emitter};

const TRAY_ICON_ID: &str = "counter-status";


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

impl StatusObserver for TauriFrontend {
    fn on_update(&self, data: CounterData) {
        let count = data.count;
        if let Err(e) = self.handle.emit("background-update", data) {
            eprintln!("Failed to emit event: {}", e);
        }

        if let Some(tray) = self.handle.tray_by_id(TRAY_ICON_ID) {
            let icon = tray_icon_for_count(count);
            if let Err(e) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", e);
            }
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
            let menu = Menu::new(app)?;
            let icon = tray_icon_for_count(0);
            let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
                .icon(icon)
                .menu(&menu)
                .tooltip("pmon")
                .build(app)?;

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
