// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{window, Builder, Manager, Window};
use tracing::info;
use web_sys::{console};
use wasm_bindgen::{JsCast, JsValue};

fn main() {
  tracing_wasm::set_as_global_default();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tauri_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    info!("[mouseover]==============================>");
}

// #[tauri::command]
// fn window_event_listener(window: Window) {
//   info!("[mouseover]==============================>");
//     // window.on_window_event(|event| {
//     //     info!("[window_event_listener]==============================>");
//     //     println!("[window_event_listener]==============================>");
//     // });

//     window.listen("mouseover", move |event| {
//         println!("window just loaded a component");
//         info!("[mouseover]==============================>");
//     });
// }

#[tauri::command]
async fn tauri_api(window: tauri::Window) {
    info!("[tauri]==============================>");
    println!("[tauri]==============================>");
    console::log_1(&JsValue::from("[tauri]===================>"));
}
