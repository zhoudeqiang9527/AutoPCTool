// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scan;
use std::option;

use scan::{scan_colors, scan_loop, scan_once, stop_scan};
use tauri::{menu::{MenuBuilder, MenuItemBuilder}};
fn main() {
    tauri::Builder::default().setup(|app| {
        let option = MenuItemBuilder::new("option").build(app)?;
        let quite = MenuItemBuilder::new("quit").build(app)?;
        let menu = MenuBuilder::new(app).items(&[&option, &quite]).build()?;
        app.set_menu(menu)?;
        app.on_menu_event(move |app, event| {
            if event.id() == "option" {
                app.exit(0);
            } else if event.id() == "quit" {
                app.exit(0);
            }
        });
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![scan_once, scan_loop, scan_colors, stop_scan])
    .run(tauri::generate_context!())
    .expect("error running tauri application");
    
}
