use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct CarGroup {
    id: String,
    name: String,
    user_id: String,
    main: String,
    test: String,
    ngp: String,
}

#[tauri::command]
pub fn get_car_groups(app_handle: AppHandle) -> Result<String, String> {
    dotenv::dotenv().ok();

    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    let settings_json = fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    let settings: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    let dir_path = settings
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .ok_or("Missing rbr_directory in settings.json")?;

    let file_name = env::var("FILE_CAR_GROUPS")
        .map_err(|_| "Missing `FILE_CAR_GROUPS` in .env file".to_string())?;

    let full_path = Path::new(dir_path).join(file_name);

    let json_data =
        fs::read_to_string(&full_path).map_err(|e| format!("Failed to read JSON file: {}", e))?;

    let car_groups: Vec<CarGroup> =
        serde_json::from_str(&json_data).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    serde_json::to_string(&car_groups).map_err(|_| "Failed to serialize car groups".to_string())
}
