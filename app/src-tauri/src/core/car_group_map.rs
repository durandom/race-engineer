use ini::Ini;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct CarGroupMapEntry {
    group_id: u32,
    car_id: u32,
    id: u32,
    name: String,
    ngp: u8,
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

    let car_group_map_file_name = env::var("FILE_CAR_GROUP_MAP")
        .map_err(|_| "Missing `FILE_CAR_GROUP_MAP` in .env file".to_string())?;

    let car_group_map_path = Path::new(dir_path).join(car_group_map_file_name);

    let conf = Ini::load_from_file(&car_group_map_path)
        .map_err(|e| format!("Failed to load file: {}", e))?;

    let mut entries: Vec<CarGroupMapEntry> = Vec::new();

    for (_section_name, properties) in &conf {
        let group_id = match properties
            .get("group_id")
            .and_then(|v| v.parse::<u32>().ok())
        {
            Some(val) => val,
            None => continue, // Skip this section
        };
        let car_id = match properties.get("car_id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue,
        };
        let id = match properties.get("id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue,
        };
        let name = match properties.get("name") {
            Some(val) => val.to_string(),
            None => continue,
        };
        let ngp = match properties.get("ngp").and_then(|v| v.parse::<u8>().ok()) {
            Some(val) => val,
            None => continue,
        };

        entries.push(CarGroupMapEntry {
            group_id,
            car_id,
            id,
            name,
            ngp,
        });
    }

    serde_json::to_string(&entries).map_err(|_| "Failed to serialize car group map".to_string())
}
