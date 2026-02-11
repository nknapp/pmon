mod use_cases;

use crate::use_cases::counter::{CounterData, MonitoringService, StatusObserver};
use tauri::{image::Image, menu::Menu, tray::TrayIconBuilder, AppHandle, Emitter};

const TRAY_ICON_ID: &str = "counter-status";

fn tray_icon_for_count(count: u32) -> Image<'static> {
    const SIZE: u32 = 32;
    const RED: [u8; 3] = [220, 50, 50];
    const GREEN: [u8; 3] = [46, 186, 86];
    const BLUE: [u8; 3] = [60, 120, 230];

    let (left, right, split) = if count % 6 == 0 {
        (RED, BLUE, true)
    } else if count % 3 == 0 {
        (GREEN, BLUE, true)
    } else if count % 2 == 0 {
        (RED, RED, false)
    } else {
        (GREEN, GREEN, false)
    };

    let mut rgba = vec![0u8; (SIZE * SIZE * 4) as usize];
    let center = (SIZE as f32 - 1.0) / 2.0;
    let radius = SIZE as f32 / 2.0 - 1.0;
    let radius_sq = radius * radius;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            if dx * dx + dy * dy <= radius_sq {
                let color = if split && x < SIZE / 2 { left } else { right };
                let idx = ((y * SIZE + x) * 4) as usize;
                rgba[idx] = color[0];
                rgba[idx + 1] = color[1];
                rgba[idx + 2] = color[2];
                rgba[idx + 3] = 255;
            }
        }
    }

    Image::new_owned(rgba, SIZE, SIZE)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

impl StatusObserver for TauriFrontend {
    fn on_update(&self, data: CounterData) {
        let count = data.count;
        if let Err(e) = self.handle.emit("background-update", data) {
            eprintln!("Failed to emit event: {}", e);
        }

        if let Some(tray) = self.handle.tray_by_id(TRAY_ICON_ID) {
            let icon = tray_icon_for_count(count);
            if let Err(e) = tray.set_icon(Some(icon)) {
                eprintln!("Failed to update tray icon: {}", e);
            }
        }
    }
}

#[derive(Default)]
struct AppState {}

struct TauriFrontend {
    handle: AppHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let menu = Menu::new(app)?;
            let icon = tray_icon_for_count(0);
            let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
                .icon(icon)
                .menu(&menu)
                .tooltip("pmon")
                .build(app)?;

            let _app_handle = app.handle().clone();
            let service = MonitoringService::new(Box::new(TauriFrontend {
                handle: app.handle().clone(),
            }));
            service.create_counter();
            // Spawn background thread that runs every 2 seconds
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
