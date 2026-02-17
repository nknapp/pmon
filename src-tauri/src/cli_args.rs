use std::path::PathBuf;
use tauri_plugin_cli::CliExt;

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub(crate) edit_config: bool,
    pub(crate) config_file: Option<PathBuf>,
}

pub fn cli_args_from_app(app: &tauri::App) -> Result<CliArgs, Box<dyn std::error::Error + 'static>> {
    let matches = app.cli().matches()?;
    let args = matches.args;
    let edit_config = args
        .get("edit-config")
        .map(|arg| arg.occurrences > 0)
        .unwrap_or(false);
    let config_file = args
        .get("config-file")
        .and_then(|arg| arg.value.as_str())
        .map(PathBuf::from);


    Ok(CliArgs {
        edit_config,
        config_file,
    })
}
