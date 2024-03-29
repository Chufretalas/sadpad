#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::{FileDialog};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn open_file() -> (String, String) {
    //returns the file name and its contents
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("text file", &["txt"])
        .show_open_single_file()
        .unwrap();

    match path {
        Some(p) => {
            (format!("{:?}", p.as_path()), fs::read_to_string(p).unwrap()) //TODO: error treat this
        }
        None => ("".to_string(), "".to_string()),
    }
}

#[tauri::command]
fn save_file(path_str: &str, content: &str) -> String { // returns "" if nothing gets saved
    let has_file_open = Path::new(path_str).exists();
    let final_path;
    if !has_file_open {
        let result = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("text file", &["txt"])
            .show_save_single_file()
            .unwrap();
        match result {
            Some(p) => final_path = p,
            None => return "".to_string(),
        }
    } else {
        final_path = PathBuf::from_str(path_str).unwrap();
    }

    fs::write(&final_path, content).unwrap();

    format!("{:?}", final_path.as_path())
}
