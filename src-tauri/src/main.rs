// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scan;
use scan::{scan_colors, scan_loop, scan_once, stop_scan};

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![scan_once, scan_loop, scan_colors, stop_scan])
    .run(tauri::generate_context!())
    .expect("error running tauri application");
    
}
