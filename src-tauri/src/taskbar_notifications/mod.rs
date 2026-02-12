mod tray_icon;

use crate::core::{NotificationState, NotificationStateController};
use tauri::AppHandle;
use tray_icon::tray_icon;

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
