pub mod core;
pub mod data_providers;
mod tray_icon;

mod config_file;
mod setup;
mod cli_args;

use crate::setup::setup;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

