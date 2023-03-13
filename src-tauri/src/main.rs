#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dir_command;
mod exec_command;

use std::{env};
use dir_command::{ get_dirs };
use exec_command::{ init_pro };

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dirs, init_pro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
