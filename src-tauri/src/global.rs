use std::{fs, path};

pub fn get_app_config_dir_path() -> path::PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    let app_config_dir = config_dir.join("netbalance");

    if !app_config_dir.exists() {
        fs::create_dir(&app_config_dir).unwrap();
    }

    app_config_dir
}

pub fn get_app_db_file_path() -> path::PathBuf {
    let app_config_dir = get_app_config_dir_path();

    let file = app_config_dir.join("netbalance.sqlite3");

    if !file.exists() {
        fs::File::create(&file).unwrap();
    }

    file
}

pub fn get_app_log_file_path() -> path::PathBuf {
    let app_config_dir = get_app_config_dir_path();

    let file = app_config_dir.join("netbalance.log");

    if !file.exists() {
        fs::File::create(&file).unwrap();
    }

    file
}
