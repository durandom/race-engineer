use ini::Ini;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    key: String,
    value: String,
}

#[tauri::command]
pub fn get_settings(app_handle: AppHandle) -> Result<String, String> {
    dotenv::dotenv().ok();

    let store_path = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json");

    let settings_json = fs::read_to_string(store_path)
        .map_err(|e| format!("Failed to read settings.json: {}", e))?;

    let settings_dir: serde_json::Value = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings.json: {}", e))?;

    let dir_path = settings_dir
        .get("rbr_directory")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let settings_file_name =
        env::var("FILE_SETTINGS").map_err(|_| "Missing `settings` in .env file".to_string())?;

    let settings_ini_path = Path::new(dir_path).join(settings_file_name);

    let conf = Ini::load_from_file(settings_ini_path)
        .map_err(|_| "Failed to load the settings.ini file".to_string())?;

    if let Some(settings) = conf.section(Some("Settings")) {
        let settings_data: Vec<Settings> = settings
            .iter()
            .filter_map(|(key, value)| {
                if !key.starts_with('#') && !key.starts_with(';') {
                    Some(Settings {
                        key: key.to_string(),
                        value: value.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        serde_json::to_string(&settings_data)
            .map_err(|_| "Failed to serialize settings".to_string())
    } else {
        Err("No [Settings] section found in the settings.ini file".to_string())
    }
}
