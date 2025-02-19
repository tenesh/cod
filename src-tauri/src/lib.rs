mod setup;
mod utils;
mod database;
mod constant;

use std::sync::Mutex;
use tauri::async_runtime::spawn;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(setup::SetupState {
            database_task: false,
            directory_task: false,
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            spawn(setup::initialize(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running application");
}