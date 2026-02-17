pub mod core;
pub mod data_providers;
mod tray_icon;

mod config_file;
mod setup;

use crate::setup::setup;
use clap::Parser;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args = CliArgs::parse();
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| setup(app, args.clone()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Parser, Debug, Clone)]
#[command(name = "pmon", version, about = "GitHub Workflow Monitor")]
pub struct CliArgs {
    #[arg(long, help = "Open $EDITOR with the config file")]
    pub(crate) edit_config: bool,
    #[arg(long, value_name = "PATH", help = "Use an alternative config file")]
    pub(crate) config_file: Option<PathBuf>,
}
