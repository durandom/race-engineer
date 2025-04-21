use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct CarGroupMapEntry {
    group_id: String,
    car_id: String,
    id: String,
    name: String,
    ngp: String,
}

#[tauri::command]
pub fn get_car_group_map(app_handle: AppHandle) -> Result<String, String> {
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

    let car_group_map_path = Path::new(dir_path).join("car_group_map.json");

    let json_string = fs::read_to_string(&car_group_map_path)
        .map_err(|e| format!("Failed to read car_group_map.json: {}", e))?;

    let entries: Vec<CarGroupMapEntry> = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse car_group_map.json: {}", e))?;

    serde_json::to_string(&entries).map_err(|_| "Failed to serialize car group map".to_string())
}
