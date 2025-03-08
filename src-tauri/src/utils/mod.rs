pub mod constants;
pub mod events;

use anyhow::Result;
use dirs::home_dir;
use std::path;

pub fn get_app_dir_path() -> Result<path::PathBuf> {
    home_dir()
        .map(|home| home.join(constants::DIRECTORY_NAME))
        .ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))
}

pub fn get_log_file_path() -> Result<path::PathBuf> {
    Ok(get_app_dir_path()?.join(constants::LOG_FILE))
}

pub fn get_config_file_path() -> Result<path::PathBuf> {
    Ok(get_app_dir_path()?.join(constants::CONFIG_FILE))
}
