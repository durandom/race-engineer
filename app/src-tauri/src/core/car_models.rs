use ini::Ini;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarModel {
    id: u32,
    name: String,
    path: String,
    filename: String,
    hash: String,
    fallback: u32,
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

    let conf =
        Ini::load_from_file(&car_models_path).map_err(|e| format!("Failed to load file: {}", e))?;

    let mut car_models: Vec<CarModel> = Vec::new();

    for (_section_name, properties) in &conf {
        let id = match properties.get("id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue,
        };
        let name = match properties.get("name") {
            Some(val) => val.to_string(),
            None => continue,
        };
        let path = match properties.get("path") {
            Some(val) => val.to_string(),
            None => continue,
        };
        let filename = match properties.get("filename") {
            Some(val) => val.to_string(),
            None => continue,
        };
        let hash = match properties.get("hash") {
            Some(val) => val.to_string(),
            None => continue,
        };
        let fallback = match properties
            .get("fallback")
            .and_then(|v| v.parse::<u32>().ok())
        {
            Some(val) => val,
            None => continue,
        };

        car_models.push(CarModel {
            id,
            name,
            path,
            filename,
            hash,
            fallback,
        });
    }

    serde_json::to_string(&car_models).map_err(|_| "Failed to serialize car models".to_string())
}
