// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;
use std::path::Path;

#[tauri::command]
fn run_denoiser() -> Result<String, String> {
    // Relative path to the file
    let relative_path = "denoiser\\denoiser";
    
    // Convert the relative path to an absolute path
    let absolute_path = std::env::current_dir().expect("Failed to get current directory")
        .join(relative_path);
    
    // Convert the absolute path to a string
    let path_str = absolute_path.to_str().expect("Invalid Unicode in the path");
    
    let output = Command::new(path_str)
        .output()
        .map_err(|e| format!("Failed to execute denoiser: {:?}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Error executing denoiser: {:?}", output.stderr))
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_denoiser])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
