// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{window, Builder, Manager, Window};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tauri_api, tauri_set_window_decorations_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn tauri_api(window: tauri::Window) {
    println!("[tauri]==============================>");
}

#[tauri::command]
fn tauri_set_window_decorations_api(window: Window, decorations: bool) {
    println!("[tauri_set_window_decoration_api]==============================>{:?}", decorations);
    window.set_decorations(decorations);
}