use ini::Ini;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct CarGroup {
    id: u32,
    name: String,
    user_id: u32,
    main: u32,
    test: u32,
    ngp: u8,
}

#[tauri::command]
pub fn get_car_groups(app_handle: AppHandle) -> Result<String, String> {
    dotenv::dotenv().ok();

    // Get the app's settings.json file
    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    // Read the settings file
    let settings_json = fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    // Parse the JSON settings
    let settings_dir: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    // Retrieve the RBR directory from the settings
    let dir_path = settings_dir
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .ok_or("Missing rbr_directory in settings.json")?;

    // Get the car groups file name from the environment variable
    let car_groups_file_name = env::var("FILE_CAR_GROUPS")
        .map_err(|_| "Missing `FILE_CAR_GROUPS` in .env file".to_string())?;

    // Construct the full path to the car groups file
    let car_groups_path = Path::new(dir_path).join(car_groups_file_name);

    // Load the INI file
    let conf =
        Ini::load_from_file(&car_groups_path).map_err(|e| format!("Failed to load file: {}", e))?;

    let mut car_groups: Vec<CarGroup> = Vec::new();

    // Parse each section in the INI file and map to CarGroup struct
    for (_section_name, properties) in &conf {
        let id = match properties.get("id").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue, // Skip this section if no valid id
        };
        let name = match properties.get("name") {
            Some(val) => val.to_string(),
            None => continue, // Skip if no name
        };
        let user_id = match properties
            .get("user_id")
            .and_then(|v| v.parse::<u32>().ok())
        {
            Some(val) => val,
            None => continue, // Skip if no valid user_id
        };
        let main = match properties.get("main").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue, // Skip if no valid main
        };
        let test = match properties.get("test").and_then(|v| v.parse::<u32>().ok()) {
            Some(val) => val,
            None => continue, // Skip if no valid test
        };
        let ngp = match properties.get("ngp").and_then(|v| v.parse::<u8>().ok()) {
            Some(val) => val,
            None => continue, // Skip if no valid ngp
        };

        // Push the parsed data into the result vector
        car_groups.push(CarGroup {
            id,
            name,
            user_id,
            main,
            test,
            ngp,
        });
    }

    // Serialize the car_groups vector into a JSON string and return it
    serde_json::to_string(&car_groups).map_err(|_| "Failed to serialize car groups".to_string())
}
