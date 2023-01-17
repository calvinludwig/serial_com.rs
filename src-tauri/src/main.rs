#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serial_com::SerialCom;
use std::thread;
use tauri::Window;

mod serial_com;

#[tauri::command]
fn start_com(window: Window) {
    thread::spawn(move || {
        SerialCom::new(window).start();
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_com])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
