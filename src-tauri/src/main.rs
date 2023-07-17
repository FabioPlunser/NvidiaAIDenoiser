// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;

#[tauri::command]
fn run_denoiser(args: String) -> String {
    let mut denoiser_cmd = Command::new("./denoiser/denoiser");
    // Split the arguments string into separate arguments
    let arguments: Vec<&str> = args.split_whitespace().collect();
    // Set the arguments for the denoiser command
    denoiser_cmd.args(&arguments);

    // Execute the denoiser command and capture the output
    let output = denoiser_cmd.output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // If the command executed successfully, return the stdout
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                // If the command failed, return an error message or handle the error as needed
                format!("Error executing denoiser command: {:?}", output.stderr)
            }
        }
        Err(e) => format!("Error executing denoiser command: {:?}", e),
    }
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_denoiser])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
