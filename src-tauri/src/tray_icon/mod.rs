mod tray_icon;

use std::sync::Arc;

use crate::core::{StateSummary, StateSummaryDispatcher, StateSummarySink};
use tauri::{menu::Menu, tray::TrayIconBuilder, AppHandle};
use tray_icon::tray_icon;

type DispatcherHandle = Arc<StateSummaryDispatcher>;

pub fn init_with(dispatcher: DispatcherHandle) -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("tray-icon")
        .setup(move |app, _| Ok(setup_with(app, &dispatcher)?))
        .build()
}

struct TrayIconController {
    handle: AppHandle,
    tray_id: String,
}

const TRAY_ICON_ID: &str = "counter-status";

impl TrayIconController {
    fn new(handle: AppHandle, tray_id: impl Into<String>) -> Self {
        Self {
            handle,
            tray_id: tray_id.into(),
        }
    }
}

impl StateSummarySink for TrayIconController {
    fn set_state_summary(&self, state: StateSummary) {
        let icon = tray_icon(state);
        if let Some(tray) = self.handle.tray_by_id(&self.tray_id) {
            if let Err(error) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", error);
            }
        }
    }
}

fn setup_with(app: &AppHandle, dispatcher: &DispatcherHandle) -> Result<(), tauri::Error> {
    setup_tray(app)?;
    dispatcher.add_controller(create_controller(app.clone()));
    Ok(())
}

fn setup_tray(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = Menu::new(app)?;
    let icon = tray_icon(StateSummary::Ok);
    let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(icon)
        .menu(&menu)
        .tooltip("pmon")
        .build(app)?;
    Ok(())
}

fn create_controller(handle: AppHandle) -> Box<dyn StateSummarySink> {
    Box::new(TrayIconController::new(handle, TRAY_ICON_ID))
}
