use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarData {
    car_id: Option<String>,         // Changed to Option<String>
    power: Option<String>,          // Changed to Option<String>
    torque: Option<String>,         // Changed to Option<String>
    drive_train: Option<String>,    // Changed to Option<String>
    engine: Option<String>,         // Changed to Option<String>
    transmission: Option<String>,   // Changed to Option<String>
    weight: Option<String>,         // Changed to Option<String>
    wdf: Option<String>,            // Changed to Option<String>
    steering_wheel: Option<String>, // Changed to Option<String>
    skin: Option<String>,           // Changed to Option<String>
    model: Option<String>,          // Changed to Option<String>
    audio: Option<String>,          // Changed to Option<String>
    year: Option<String>,           // Changed to Option<String>
    shifter_type: Option<String>,   // Changed to Option<String>
    id: Option<String>,             // Changed to Option<String>
}

#[tauri::command]
pub fn get_car_data(app_handle: AppHandle) -> Result<String, String> {
    dotenv::dotenv().ok();

    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    let settings_json = fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    let settings_dir: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    let dir_path = settings_dir
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .ok_or("Missing rbr_directory in settings.json")?;

    let car_data_file_name = env::var("FILE_CARS_DATA")
        .map_err(|_| "Missing `FILE_CARS_DATA` in .env file".to_string())?;

    let car_data_path = Path::new(dir_path).join(car_data_file_name);

    let raw = fs::read_to_string(&car_data_path)
        .map_err(|e| format!("Failed to read car data JSON file: {}", e))?;

    let car_data_list: Vec<CarData> =
        serde_json::from_str(&raw).map_err(|e| format!("Failed to parse car data JSON: {}", e))?;

    serde_json::to_string(&car_data_list)
        .map_err(|e| format!("Failed to serialize car data: {}", e))
}
