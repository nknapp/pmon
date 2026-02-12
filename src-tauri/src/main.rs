// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod taskbar_notifications;
pub mod core;


fn main() {
    pmon_lib::run()
}
