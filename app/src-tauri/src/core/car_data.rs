use ini::Ini;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarData {
    car_id: u32,
    power: Option<String>,
    torque: Option<String>,
    drive_train: Option<String>,
    engine: Option<String>,
    transmission: Option<String>,
    weight: Option<String>,
    wdf: Option<String>,
    steering_wheel: Option<String>,
    skin: Option<String>,
    model: Option<String>,
    audio: Option<String>,
    year: Option<String>,
    shifter_type: Option<String>,
    id: u32,
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
        .map_err(|_| "Missing `FILE_CAR_DATA` in .env file".to_string())?;

    let car_data_path = Path::new(dir_path).join(car_data_file_name);

    let conf =
        Ini::load_from_file(&car_data_path).map_err(|e| format!("Failed to load file: {}", e))?;

    let mut car_data_list: Vec<CarData> = Vec::new();

    for (_section, properties) in &conf {
        let car_id = match properties.get("car_id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue,
        };

        let id = match properties.get("id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue,
        };

        car_data_list.push(CarData {
            car_id,
            power: properties.get("power").map(|s| s.to_string()),
            torque: properties.get("torque").map(|s| s.to_string()),
            drive_train: properties.get("drive_train").map(|s| s.to_string()),
            engine: properties.get("engine").map(|s| s.to_string()),
            transmission: properties.get("transmission").map(|s| s.to_string()),
            weight: properties.get("weight").map(|s| s.to_string()),
            wdf: properties.get("wdf").map(|s| s.to_string()),
            steering_wheel: properties.get("steering_wheel").map(|s| s.to_string()),
            skin: properties.get("skin").map(|s| s.to_string()),
            model: properties.get("model").map(|s| s.to_string()),
            audio: properties.get("audio").map(|s| s.to_string()),
            year: properties.get("year").map(|s| s.to_string()),
            shifter_type: properties.get("shifterType").map(|s| s.to_string()),
            id,
        });
    }

    serde_json::to_string(&car_data_list).map_err(|_| "Failed to serialize car data".to_string())
}
