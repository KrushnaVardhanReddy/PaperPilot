// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Spec 160: Tauri command `run_system_doctor` is registered in desktop_lib::run invoke_handler

fn main() {
    desktop_lib::run()
}
