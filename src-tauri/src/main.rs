// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod tray_icon;
pub mod core;

fn main() {
    pmon_lib::run()
}
