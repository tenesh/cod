use std::thread::sleep;
use std::time::Duration;
use anyhow::{Result};

use crate::database;
use crate::global::{get_app_config_dir_path, get_app_db_file_path, get_app_log_file_path};

pub fn setup_logger() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        let console_writer = std::io::stderr;

        let subscriber = tracing_subscriber::fmt()
            .with_writer(console_writer)
            .with_max_level(tracing::Level::DEBUG)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to setup logger for console");
    }

    #[cfg(not(debug_assertions))]
    {
        let log_file = crate::global::get_app_log_file_path();

        let file_writer = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file);

        let subscriber = tracing_subscriber::fmt()
            .with_writer(file_writer)
            .with_max_level(tracing::Level::INFO)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to setup logger for file");
    }

    info!("Logger initialized successfully");

    Ok(())
}
pub fn setup_app() -> Result<()> {
    let app_dir = get_app_config_dir_path();
    let db_file = get_app_db_file_path();
    let log_file = get_app_log_file_path();

    info!("Application config directory: {:?}", app_dir);
    info!("Database file: {:?}", db_file);
    info!("Log file: {:?}", log_file);

    database::run_migrations()?;

    sleep(Duration::from_secs(25));

    info!("Database migrations completed successfully");

    Ok(())
}
