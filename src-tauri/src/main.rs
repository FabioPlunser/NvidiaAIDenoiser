// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::process::Command;

#[tauri::command]
fn run_denoiser(args: String) -> Result<String, String> {
    println!("Running denoiser with args: {}", args);
    // Relative path to the file
    let relative_path = "denoiser\\denoiser";

    // Convert the relative path to an absolute path
    let absolute_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(relative_path);

    // Convert the absolute path to a string
    let path_str = absolute_path.to_str().expect("Invalid Unicode in the path");

    let mut denoiser = Command::new(path_str);

    let arguments: Vec<&str> = args.split_whitespace().collect();

    denoiser.args(arguments);

    let output = denoiser
        .output()
        .map_err(|e| format!("Failed to execute denoiser: {:?}", e))?;

    if output.status.success() {
        // If the command executed successfully, return the stdout
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // If the command failed, return an error message or handle the error as needed
        Err(format!("Error executing denoiser command: {:?}", output.stderr))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_denoiser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
