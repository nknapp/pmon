use crate::config_file::read_config;
use crate::core::{DataProvider, StateSummaryGateway};
use crate::data_providers::providers_from_config;
use crate::tray_icon;
use crate::CliArgs;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{App, AppHandle, Manager};

pub fn setup(app: &mut App, args: CliArgs) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();
    let config_file = resolve_config_file(&args, &app_handle)?;
    if args.edit_config {
        edit_config_file(app_handle, &config_file)?;
        return Ok(());
    }
    let mut providers = load_providers(&config_file);
    let state_summary_gateway = Arc::new(StateSummaryGateway::new());

    tray_icon::setup_with(&app_handle, &state_summary_gateway)?;

    for provider in &mut providers {
        provider.start(state_summary_gateway.clone());
    }

    Ok(())
}

fn edit_config_file(
    app_handle: &AppHandle,
    config_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    ensure_config_exists(&config_file)?;
    open_editor(&config_file)?;
    app_handle.exit(0);
    Ok(())
}

fn resolve_config_file(args: &CliArgs, app_handle: &AppHandle) -> Result<PathBuf, Box<dyn Error>> {
    Ok(match args.config_file.clone() {
        Some(path) => path,
        None => default_config_path(&app_handle)?,
    })
}

fn load_providers(config_file: &PathBuf) -> Vec<Box<dyn DataProvider>> {
    match read_config(config_file) {
        Ok(config) => providers_from_config(&config),
        Err(error) => {
            log::error!("Failed to load config.yaml: {}", error);
            Err(error).unwrap()
        }
    }
}

fn default_config_path(
    app_handle: &tauri::AppHandle,
) -> Result<PathBuf, Box<dyn std::error::Error + 'static>> {
    let config_dir = match app_handle.path().config_dir() {
        Ok(path) => path,
        Err(e) => return Err(e.into()),
    };
    Ok(config_dir.join("config.yaml"))
}

fn ensure_config_exists(config_file: &PathBuf) -> Result<(), Box<dyn std::error::Error + 'static>> {
    read_config(config_file)?;
    Ok(())
}

fn open_editor(config_file: &PathBuf) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let editor = std::env::var("EDITOR")
        .map_err(|_| "EDITOR is not set. Set $EDITOR to open the config file.")?;
    let status = std::process::Command::new(editor)
        .arg(config_file)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err("Editor exited with a non-zero status".into())
    }
}
