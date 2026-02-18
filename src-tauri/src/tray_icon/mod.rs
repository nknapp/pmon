mod tray_icon;

use std::sync::Arc;

use crate::core::{StateSummary, StateSummaryAdapter, StateSummaryGateway};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tray_icon::tray_icon;

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

impl StateSummaryAdapter for TrayIconController {
    fn set_state_summary(&self, state: StateSummary) {
        let icon = tray_icon(state);
        if let Some(tray) = self.handle.tray_by_id(&self.tray_id) {
            if let Err(error) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", error);
            }
        }
    }
}

pub fn setup_with(
    app: &AppHandle,
    dispatcher: &Arc<StateSummaryGateway>,
) -> Result<(), tauri::Error> {
    setup_tray(app)?;
    dispatcher.add_controller(create_controller(app.clone()));
    Ok(())
}

fn setup_tray(app: &AppHandle) -> Result<(), tauri::Error> {
    let open_window = MenuItem::with_id(app, "open-window", "Open window", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_window, &exit])?;
    let icon = tray_icon(StateSummary::Ok);
    let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(icon)
        .menu(&menu)
        .tooltip("pmon")
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open-window" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            "exit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}

fn create_controller(handle: AppHandle) -> Box<dyn StateSummaryAdapter> {
    Box::new(TrayIconController::new(handle, TRAY_ICON_ID))
}
