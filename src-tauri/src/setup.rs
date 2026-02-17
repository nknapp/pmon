use std::path::{PathBuf};
use std::sync::Arc;
use tauri::{App, Manager};
use crate::config_file::read_config;
use crate::core::{DataProvider, StateSummaryGateway};
use crate::data_providers::providers_from_config;
use crate::tray_icon;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>>{
    let app_handle = app.handle();
    let config_dir = match app_handle.path().config_dir() {
        Ok(path) => path,
        Err(e) => return Err(e.into()),
    };
    let config_file = config_dir.join("config.yaml");
    let mut providers = load_providers(&config_file);
    let state_summary_gateway = Arc::new(StateSummaryGateway::new());

    tray_icon::setup_with(&app_handle, &state_summary_gateway)?;

    for provider in &mut providers {
        provider.start(state_summary_gateway.clone());
    }

    Ok(())
}

fn load_providers(config_file: &PathBuf) -> Vec<Box<dyn DataProvider>> {
    eprintln!("Loading providers from config.yaml in directory {}", std::env::current_dir().unwrap().display());
    match read_config(config_file) {
        Ok(config) => providers_from_config(&config),
        Err(error) => {
            eprintln!("Failed to load config.yaml: {}", error);
            Vec::new()
        }
    }
}