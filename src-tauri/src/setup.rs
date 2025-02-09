use super::{constant, utils};
use std;
use tauri::{AppHandle, Manager};

pub struct SetupState {
    pub database_task: bool,
    pub directory_task: bool,
}

pub async fn initialize(app: AppHandle) -> Result<(), ()> {
    let state = app.state::<std::sync::Mutex<SetupState>>();

    if let Err(e) = create_app_directory(&state) {
        eprintln!("Failed to create app directory: {}", e);
        return Err(());
    }

    if let Err(e) = create_database_file(&state) {
        eprintln!("Failed to create database file: {}", e);
        return Err(());
    }

    let state_lock = state.lock().unwrap();

    if state_lock.database_task && state_lock.directory_task {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

fn create_app_directory(state: &std::sync::Mutex<SetupState>) -> std::io::Result<()> {
    let home_dir = utils::get_home_directory().unwrap();

    let app_dir_path = format!("{}/{}", home_dir, constant::APP_DIRECTORY_NAME,);

    let path = std::path::Path::new(&app_dir_path);

    std::fs::create_dir_all(&path)?;

    println!("App directory created at: {}", path.display());

    let mut state_lock = state.lock().unwrap();
    state_lock.directory_task = true;

    Ok(())
}

pub fn create_database_file(state: &std::sync::Mutex<SetupState>) -> std::io::Result<()> {
    let app_dir_path = utils::get_app_directory();
    let db_file_path = format!(
        "{}/{}.{}",
        app_dir_path,
        constant::APP_DATABASE_NAME,
        "sqlite"
    );

    let path = std::path::Path::new(&db_file_path);

    if !path.exists() {
        std::fs::File::create(path)?;
        println!("Database file created at: {}", path.display());
    } else {
        println!("Database file already exists at: {}", path.display());
    }

    let mut state_lock = state.lock().unwrap();
    state_lock.database_task = true;

    Ok(())
}
