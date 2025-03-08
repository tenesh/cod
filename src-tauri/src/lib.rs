extern crate core;

mod api;
mod database;
mod utils;

use crate::utils::{constants, events, get_app_dir_path};
use anyhow::{Context, Error, Result};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use std::fs;
use std::sync;
use tauri::async_runtime;
use tauri::{AppHandle, Emitter, Manager, State};
use tracing_subscriber;

pub struct AppState {
    pub init: bool,
    pub pool: Option<Pool<ConnectionManager<SqliteConnection>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pool: None,
            init: false,
        }
    }

    pub fn set_pool(&mut self, pool: Pool<ConnectionManager<SqliteConnection>>) {
        self.pool = Some(pool);
    }

    pub fn set_init(&mut self, init: bool) {
        self.init = init;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(sync::Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            async_runtime::spawn(setup(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running application");
}

async fn setup(app: AppHandle) -> Result<()> {
    let clone = app.clone();
    let state = app.state::<sync::Mutex<AppState>>();

    let app_dir = get_app_dir_path().context("Failed to get application directory path")?;

    let db_file = app_dir.join(constants::DATABASE_FILE);
    let log_file = app_dir.join(constants::LOG_FILE);
    let config_file = app_dir.join(constants::CONFIG_FILE);

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .context("Failed to create application directory")
            .map_err(|e| app.emit(events::EVENT_SETUP_FAILED, e.to_string()))?;
    }

    if !db_file.exists() {
        fs::File::create(&db_file)
            .context("Failed to create database file")
            .map_err(|e| app.emit(events::EVENT_SETUP_FAILED, e.to_string()))?;
    }

    if !log_file.exists() {
        fs::File::create(&log_file)
            .context("Failed to create log file")
            .map_err(|e| app.emit(events::EVENT_SETUP_FAILED, e.to_string()))?;
    }

    if !config_file.exists() {
        fs::File::create(&config_file)
            .context("Failed to create config file")
            .map_err(|e| app.emit(events::EVENT_SETUP_FAILED, e.to_string()))?;
    }

    if let Err(e) = setup_logger() {
        app.emit(events::EVENT_SETUP_FAILED, e.to_string())?;
        return Ok(());
    }

    database::run_migrations()
        .map_err(|e| app.emit(events::EVENT_DB_MIGRATIONS_FAILED, e.to_string()))?;

    let pool = database::get_connection_pool()
        .map_err(|e| app.emit(events::EVENT_DB_CONNECTION_POOL_FAILED, e.to_string()))?;

    complete(clone, state, pool);
    Ok(())
}

fn complete(
    app: AppHandle,
    state: State<'_, sync::Mutex<AppState>>,
    pool: Pool<ConnectionManager<SqliteConnection>>,
) {
    let mut state_lock = state.lock().unwrap();

    state_lock.set_init(true);
    state_lock.set_pool(pool);

    if state_lock.init {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
}

fn setup_logger() -> Result<(), Error> {
    let log_file = utils::get_log_file_path().context("Failed to get log file path.")?;

    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .context("Failed to open log file.")?;

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file)
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).context("Failed to set logger.")?;

    Ok(())
}
