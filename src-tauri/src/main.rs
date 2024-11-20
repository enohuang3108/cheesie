// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use enigo::*;

#[tauri::command]
fn move_mouse(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![move_mouse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
