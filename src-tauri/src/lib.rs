pub mod core;
pub mod data_providers;
mod tray_icon;
mod use_cases;

mod setup;
mod config_file;

use crate::setup::setup;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


