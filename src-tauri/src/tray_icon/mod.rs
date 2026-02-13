mod tray_icon;

use std::sync::{Arc, Mutex};

use crate::core::{StateSummary, StateSummaryDispatcher, StateSummarySink};
use tauri::{menu::Menu, tray::TrayIconBuilder, AppHandle, Manager};
use tray_icon::tray_icon;

pub const TRAY_ICON_ID: &str = "counter-status";

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
    Box::new(TaskbarNotificationStateController::new(
        handle,
        TRAY_ICON_ID,
    ))
}

pub fn init() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("tray-icon")
        .setup(|app, _| {
            setup_tray(app)?;
            let dispatcher = app
                .state::<Arc<Mutex<StateSummaryDispatcher>>>()
                .inner()
                .clone();
            let mut dispatcher = dispatcher.lock().expect("dispatcher lock");
            dispatcher.add_controller(create_controller(app.app_handle().clone()));
            Ok(())
        })
        .build()
}

pub struct TaskbarNotificationStateController {
    handle: AppHandle,
    tray_id: String,
}

impl TaskbarNotificationStateController {
    pub fn new(handle: AppHandle, tray_id: impl Into<String>) -> Self {
        Self {
            handle,
            tray_id: tray_id.into(),
        }
    }
}

impl StateSummarySink for TaskbarNotificationStateController {
    fn set_state_summary(&self, state: StateSummary) {
        let icon = tray_icon(state);
        if let Some(tray) = self.handle.tray_by_id(&self.tray_id) {
            if let Err(error) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", error);
            }
        }
    }
}
