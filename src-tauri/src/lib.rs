pub mod core;
pub mod data_providers;
mod tray_icon;
mod use_cases;

use std::sync::Arc;

use crate::core::config::read_config;
use crate::core::{DataProvider, StateSummaryGateway};
use crate::data_providers::providers_from_config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppState {
    providers: Vec<Box<dyn DataProvider>>,
}

impl AppState {
    fn new(providers: Vec<Box<dyn DataProvider>>) -> Self {
        Self { providers }
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        for provider in &mut self.providers {
            provider.stop();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state_summary_gateway = Arc::new(StateSummaryGateway::new());
    let mut providers = load_providers();
    for provider in &mut providers {
        provider.start(state_summary_gateway.clone());
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tray_icon::init_with(state_summary_gateway.clone()))
        .manage(AppState::new(providers))
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_providers() -> Vec<Box<dyn DataProvider>> {
    match read_config("config.yaml") {
        Ok(config) => providers_from_config(&config),
        Err(error) => {
            eprintln!("Failed to load config.yaml: {}", error);
            Vec::new()
        }
    }
}
