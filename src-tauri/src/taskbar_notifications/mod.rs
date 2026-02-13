mod tray_icon;

use crate::core::{NotificationState, NotificationStateController};
use tauri::{menu::Menu, tray::TrayIconBuilder, App, AppHandle};
use tray_icon::tray_icon;

pub const TRAY_ICON_ID: &str = "counter-status";

pub fn setup_tray(app: &App) -> Result<(), tauri::Error> {
    let menu = Menu::new(app)?;
    let icon = tray_icon(NotificationState::Ok);
    let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(icon)
        .menu(&menu)
        .tooltip("pmon")
        .build(app)?;
    Ok(())
}

pub fn create_controller(handle: AppHandle) -> Box<dyn NotificationStateController> {
    Box::new(TaskbarNotificationStateController::new(
        handle,
        TRAY_ICON_ID,
    ))
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

impl NotificationStateController for TaskbarNotificationStateController {
    fn set_notification_state(&self, state: NotificationState) {
        let icon = tray_icon(state);
        if let Some(tray) = self.handle.tray_by_id(&self.tray_id) {
            if let Err(error) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", error);
            }
        }
    }
}
