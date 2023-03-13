use tauri::InvokeError;
use std::{fs};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_dirs(path: &str) -> Result<Vec<String>, InvokeError>{
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
    return Ok(result.into());
}