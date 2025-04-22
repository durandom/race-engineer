use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs, path::Path};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct StageData {
    stage_id: Option<String>,      // Changed to Option<String>
    name: Option<String>,          // Changed to Option<String>
    deftime: Option<String>,       // Changed to Option<String>
    length: Option<String>,        // Changed to Option<String>
    surface_id: Option<String>,    // Changed to Option<String>
    short_country: Option<String>, // Changed to Option<String>
    author: Option<String>,        // Changed to Option<String>
    tarmac: Option<String>,        // Changed to Option<String>
    gravel: Option<String>,        // Changed to Option<String>
    snow: Option<String>,          // Changed to Option<String>
    new_update: Option<String>,    // Changed to Option<String>
    author_web: Option<String>,    // Changed to Option<String>
    author_note: Option<String>,   // Changed to Option<String>
    fattrib: Option<String>,       // Changed to Option<String>
}

#[tauri::command]
pub fn get_stages_data(app_handle: AppHandle) -> Result<String, String> {
    dotenv::dotenv().ok();

    // Path to the settings.json file
    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    // Read settings.json file
    let settings_json = fs::read_to_string(store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    // Parse the settings.json content
    let settings_dir: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    // Get the directory for RBR data from the settings
    let dir_path = settings_dir
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    // Get the file name of the stage data JSON file from the environment
    let stages_file_name = env::var("FILE_STAGES_DATA")
        .map_err(|_| "Missing `FILE_STAGES_DATA` in .env file".to_string())?;

    let stages_json_path = Path::new(dir_path).join(stages_file_name);

    // Read the stages JSON file
    let stages_json = fs::read_to_string(&stages_json_path)
        .map_err(|e| format!("Failed to read stages data JSON file: {}", e))?;

    // Deserialize the JSON into a vector of StageData structs
    let stages: Vec<StageData> = serde_json::from_str(&stages_json)
        .map_err(|e| format!("Failed to parse stages data JSON: {}", e))?;

    // Serialize the stages vector to a JSON string
    serde_json::to_string(&stages).map_err(|e| format!("Serialization failed: {}", e))
}
