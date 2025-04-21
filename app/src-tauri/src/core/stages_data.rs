use ini::Ini;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct StageData {
    stage_id: u32,
    name: String,
    deftime: u32,
    length: u32,
    surface_id: u32,
    short_country: String,
    author: String,
    tarmac: u32,
    gravel: u32,
    snow: u32,
    new_update: u32,
    author_web: String,
    author_note: String,
    fattrib: String,
}

#[tauri::command]
pub fn get_stages_data(app_handle: AppHandle) -> Result<String, String> {
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
        env::var("FILE_STAGES_DATA").map_err(|_| "Missing `settings` in .env file".to_string())?;

    let settings_ini_path = Path::new(dir_path).join(settings_file_name);

    let conf = Ini::load_from_file(settings_ini_path)
        .map_err(|_| "Failed to load the settings.ini file".to_string())?;
    let mut stages: Vec<StageData> = Vec::new();

    for (section_name, properties) in &conf {
        if section_name.is_none() || properties.is_empty() {
            continue;
        }

        let stage_id = section_name
            .map(|s| {
                s.split('_')
                    .nth(1)
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        let stage = StageData {
            stage_id,
            name: properties.get("name").unwrap_or("N/A").to_string(),
            deftime: properties
                .get("deftime")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            length: properties.get("length").unwrap_or("0").parse().unwrap_or(0),
            surface_id: properties
                .get("surface_id")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            short_country: properties.get("short_country").unwrap_or("N/A").to_string(),
            author: properties.get("author").unwrap_or("N/A").to_string(),
            tarmac: properties.get("tarmac").unwrap_or("0").parse().unwrap_or(0),
            gravel: properties.get("gravel").unwrap_or("0").parse().unwrap_or(0),
            snow: properties.get("snow").unwrap_or("0").parse().unwrap_or(0),
            new_update: properties
                .get("new_update")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
            author_web: properties.get("author_web").unwrap_or("N/A").to_string(),
            author_note: properties.get("author_note").unwrap_or("N/A").to_string(),
            fattrib: properties.get("fattrib").unwrap_or("N/A").to_string(),
        };

        stages.push(stage);
    }
    serde_json::to_string(&stages).map_err(|e| format!("Serialization failed: {}", e))
}
