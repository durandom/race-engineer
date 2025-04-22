use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    id: String,
    name: String,
    path: String,
    hash: String,
    carmodel_id: String,
    user_id: String,
    base_group_id: String,
    test: String,
    ngp: String,
    custom_setups: String,
    rev: String,
    audio: Option<String>,
    audio_hash: String,
}

#[tauri::command]
pub fn get_car(app_handle: AppHandle) -> Result<String, String> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Get the settings.json path
    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    // Read settings.json file to get the rbr_directory
    let settings_json = fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    // Parse settings.json content
    let settings_dir: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    // Extract the RBR directory path from the JSON
    let dir_path = settings_dir
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .ok_or("Missing rbr_directory in settings.json")?;

    // Load the car JSON file name from .env
    let car_json_file_name =
        std::env::var("FILE_CARS").map_err(|_| "Missing `FILE_CARS` in .env file".to_string())?;

    // Build the full path to the car JSON file
    let car_json_path = std::path::Path::new(dir_path).join(car_json_file_name);

    // Read and parse the car JSON file
    let car_json = fs::read_to_string(&car_json_path)
        .map_err(|e| format!("Failed to read car JSON file: {}", e))?;

    let cars: Vec<Car> = serde_json::from_str(&car_json)
        .map_err(|e| format!("Failed to parse car JSON file: {}", e))?;

    // Return the JSON string
    serde_json::to_string(&cars).map_err(|e| format!("Failed to serialize car data: {}", e))
}
