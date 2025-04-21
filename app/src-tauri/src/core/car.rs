use ini::Ini;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    car_id: String, // Added car_id field
    name: String,
    power: String,
    torque: String,
    drive_train: String,
    engine: String,
    transmission: String,
    weight: String,
    wdf: String,
    steering_wheel: String,
    skin: Option<String>,
    audio: Option<String>,
    year: String,
    shifter_type: String,
    path: Option<String>,
    ngp: Option<String>,
    audio_hash: Option<String>,
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

    // Load the car.ini file from .env variable or default to empty string if not found
    let car_ini_file_name =
        env::var("FILE_CARS").map_err(|_| "Missing `FILE_CAR_INI` in .env file".to_string())?;

    // Build the full path to the car.ini file
    let car_ini_path = Path::new(dir_path).join(car_ini_file_name);

    // Load the INI file for cars
    let conf = Ini::load_from_file(&car_ini_path)
        .map_err(|e| format!("Failed to load car.ini file: {}", e))?;

    // Prepare a vector to hold the cars
    let mut cars: Vec<Car> = Vec::new();

    // Iterate through the sections in the INI file and collect car data
    for (section_name, properties) in &conf {
        // Use the section name as car_id
        let car_id = section_name
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string()); // Default to "Unknown" if None
                                                       // Convert section_name to String

        // Parse each property and create Car instances
        let name = match properties.get("name") {
            Some(val) => val.to_string(),
            None => continue, // Skip if name is missing
        };

        let power = match properties.get("power") {
            Some(val) => val.to_string(),
            None => continue, // Skip if power is missing
        };

        let torque = match properties.get("torque") {
            Some(val) => val.to_string(),
            None => continue, // Skip if torque is missing
        };

        let drive_train = match properties.get("drive_train") {
            Some(val) => val.to_string(),
            None => continue, // Skip if drive_train is missing
        };

        let engine = match properties.get("engine") {
            Some(val) => val.to_string(),
            None => continue, // Skip if engine is missing
        };

        let transmission = match properties.get("transmission") {
            Some(val) => val.to_string(),
            None => continue, // Skip if transmission is missing
        };

        let weight = match properties.get("weight") {
            Some(val) => val.to_string(),
            None => continue, // Skip if weight is missing
        };

        let wdf = match properties.get("wdf") {
            Some(val) => val.to_string(),
            None => continue, // Skip if wdf is missing
        };

        let steering_wheel = match properties.get("steering_wheel") {
            Some(val) => val.to_string(),
            None => continue, // Skip if steering_wheel is missing
        };

        let skin = properties.get("skin").map(|v| v.to_string());

        let audio = properties.get("audio").map(|v| v.to_string());

        let year = match properties.get("year") {
            Some(val) => val.to_string(),
            None => continue, // Skip if year is missing
        };

        let shifter_type = match properties.get("shifterType") {
            Some(val) => val.to_string(),
            None => continue, // Skip if shifterType is missing
        };

        let path = properties.get("path").map(|v| v.to_string());

        let ngp = properties.get("ngp").map(|v| v.to_string());

        let audio_hash = properties.get("audio_hash").map(|v| v.to_string());

        // Push the Car into the vector
        cars.push(Car {
            car_id,
            name,
            power,
            torque,
            drive_train,
            engine,
            transmission,
            weight,
            wdf,
            steering_wheel,
            skin,
            audio,
            year,
            shifter_type,
            path,
            ngp,
            audio_hash,
        });
    }

    // Serialize the cars into a JSON string and return it
    serde_json::to_string(&cars).map_err(|_| "Failed to serialize cars".to_string())
}
