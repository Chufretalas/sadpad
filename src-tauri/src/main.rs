#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::fs;

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


#[tauri::command]
fn open_file() -> String { //TODO: return the file path as well so the changes can be saved
    let path = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("text file", &["txt"])
    .show_open_single_file()
    .unwrap();

    match path {
        Some(p) => {
            fs::read_to_string(p).unwrap() //TODO: error treat this
        },
        None => "".to_string()
    }
}