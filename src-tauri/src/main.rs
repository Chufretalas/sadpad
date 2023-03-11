#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::{FileDialog, MessageDialog, MessageType};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let open_menu_item = CustomMenuItem::new("open", "Open");

    let file_submenu = Submenu::new("File", Menu::new().add_item(open_menu_item));

    let menu = Menu::new().add_submenu(file_submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => {
                let path = FileDialog::new()
                    .set_location("~/Desktop")
                    .show_open_single_file()
                    .unwrap();

                match path {
                    Some(p) => {
                        println!("{:?}", p)
                    }
                    None => (),
                }
            }

            _ => ()
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
