#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, fs};
use tauri::InvokeError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Into<InvokeError>
#[tauri::command]
fn run(path: &str) -> Result<Vec<String>, InvokeError>{
    let dir_list: fs::ReadDir = match fs::read_dir(path) {
        Ok(dir_list) => dir_list,
        Err(_) => return Err(InvokeError::from("dir read error"))
    };
    let mut result: Vec<String> = vec![];
    for dir in dir_list {
        let dir = match dir {
            Ok(dir) => dir,
            Err(_) => return Err(InvokeError::from("dir read error")),
        };
        let path_buf = dir.path();
        let str_temp = path_buf.to_str();
        if let Some(name) = str_temp {
            result.push(name.to_string());
        }
    }
    // println!("{:?}, ", result);
    return Ok(result.into());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
