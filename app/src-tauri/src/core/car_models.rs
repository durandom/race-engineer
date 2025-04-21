use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarModel {
    id: String,
    name: String,
    path: String,
    filename: String,
    hash: String,
    fallback: String,
}

#[tauri::command]
pub fn get_car_models(app_handle: AppHandle) -> Result<String, String> {
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

    let car_models_file_name = env::var("FILE_CAR_MODELS")
        .map_err(|_| "Missing `FILE_CAR_MODELS` in .env file".to_string())?;

    let car_models_path = Path::new(dir_path).join(car_models_file_name);

    let car_models_json = fs::read_to_string(&car_models_path)
        .map_err(|e| format!("Failed to read car models JSON file: {}", e))?;

    let car_models: Vec<CarModel> = serde_json::from_str(&car_models_json)
        .map_err(|e| format!("Failed to parse car models JSON: {}", e))?;

    serde_json::to_string(&car_models).map_err(|_| "Failed to serialize car models".to_string())
}
