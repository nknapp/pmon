use crate::core::NotificationState;
use tauri::image::Image;

pub fn tray_icon(state: NotificationState) -> Image<'static> {
    let count = match state {
        NotificationState::Ok => 1,
        NotificationState::Failure => 2,
        NotificationState::OkPending => 3,
        NotificationState::FailurePending => 6,
    };

    tray_icon_for_count(count)
}

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
