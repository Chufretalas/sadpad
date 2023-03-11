#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::{FileDialog, MessageDialog, MessageType};

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
fn open_file() -> String {
    let path = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("text file", &["txt"])
    .show_open_single_file()
    .unwrap();

    match path {
        Some(p) => p.to_str().unwrap().to_string(),
        None => "".to_string()
    }
}