// lib.rs
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            core::settings::get_settings,
            core::car_group_map::get_car_group_map,
            core::car_groups::get_car_groups,
            core::car_models::get_car_models,
            core::car_data::get_car_data,
            core::car::get_car,
            core::rallysimfans_personal_stage::get_stage,
            core::rallysimfans_personal_car::get_car_options,
            core::stages_data::get_stages_data,
            core::stages_data::get_stages_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
