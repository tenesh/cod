extern crate core;

mod events;
mod state;

use crate::events::EVENT_SETUP_FAILED;
use crate::state::AppState;
use anyhow::{Context, Error, Result};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use netbalance_core::database;
use netbalance_core::utils;
use std::fs;
use std::sync;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Emitter, Manager, State};
use tracing_subscriber;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(sync::Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .setup(|app| {
            spawn(setup(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running application.");
}

async fn setup(app: AppHandle) -> Result<()> {
    let clone = app.clone();
    let state = app.state::<sync::Mutex<AppState>>();

    let app_dir_exists = utils::get_app_dir()
        .map(|dir| dir.exists())
        .unwrap_or(false);

    if !app_dir_exists {
        if let Err(e) = utils::file::create_app_directory() {
            app.emit(EVENT_SETUP_FAILED, e.to_string())?;
            return Ok(());
        }

        if let Err(e) = utils::file::create_database_file() {
            app.emit(EVENT_SETUP_FAILED, e.to_string())?;
            return Ok(());
        }

        if let Err(e) = utils::file::create_log_file() {
            app.emit(EVENT_SETUP_FAILED, e.to_string())?;
            return Ok(());
        }

        if let Err(e) = utils::file::create_version_file() {
            app.emit(EVENT_SETUP_FAILED, e.to_string())?;
            return Ok(());
        }
    }

    if let Err(e) = setup_logger() {
        app.emit(EVENT_SETUP_FAILED, e.to_string())?;
        return Ok(());
    }

    if let Err(e) = database::run_migrations() {
        app.emit(EVENT_SETUP_FAILED, e.to_string())?;
        return Ok(());
    }

    let pool = match database::get_connection_pool() {
        Ok(pool) => pool,
        Err(e) => {
            app.emit(EVENT_SETUP_FAILED, e.to_string())?;
            return Ok(());
        }
    };

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

    println!("{:?}", state_lock.init);

    if state_lock.init {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
}

fn setup_logger() -> Result<(), Error> {
    let log_file = utils::get_app_log_file().context("Failed to get log file path.")?;

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

#[tauri::command]
fn my_custom_command() {
    println!("Got command from tauri!");
}
