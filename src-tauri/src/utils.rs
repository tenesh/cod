use super::constant;
use dirs::{document_dir, home_dir};

pub fn get_documents_directory() -> Option<String> {
    if let Some(documents_dir) = document_dir() {
        Some(documents_dir.to_string_lossy().to_string())
    } else {
        None
    }
}

pub fn get_home_directory() -> Option<String> {
    if let Some(home_dir) = home_dir() {
        Some(home_dir.to_string_lossy().to_string())
    } else {
        None
    }
}

pub fn get_app_directory() -> String {
    let home_dir_path = get_home_directory().unwrap();
    let app_dir_path = format!("{}/{}", home_dir_path, constant::APP_DIRECTORY_NAME);
    app_dir_path
}
