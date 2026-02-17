pub mod core;
pub mod data_providers;
mod tray_icon;

mod cli_args;
mod config_file;
mod setup;

use crate::cli_args::{load_cli_args};
use crate::setup::setup;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    let cli_args = load_cli_args();


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| setup(app, cli_args))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
