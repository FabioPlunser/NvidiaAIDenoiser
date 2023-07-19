// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn run_denoiser(args: Vec<&str>) -> String {
    println!("Running denoiser with args: {:?}", args);
    // Relative path to the file
    let relative_path = "denoiser\\denoiser";

    // Convert the relative path to an absolute path
    let absolute_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(relative_path);

    // Convert the absolute path to a string
    let path_str = absolute_path.to_str().expect("Invalid Unicode in the path");

    let mut denoiser = Command::new(path_str);
    denoiser.args(args);

    let output = denoiser
        .output()
        .expect("Failed to execute denoiser command");

    if output.status.success() {
        // If the command executed successfully, return the stdout
        println!("Denoiser command executed successfully: {:?}", &output.stdout);
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        // If the command failed, return an error message or handle the error as needed
        println!("Denoiser command executed successfully: Stderr:{:?} Stdout:{:?}",  &output.stderr,
        &output.stdout);
        format!(
            "Error executing denoiser command: Stderr:{:?} Stdout:{:?}",
            &output.stderr,
            &output.stdout
        )
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_denoiser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
