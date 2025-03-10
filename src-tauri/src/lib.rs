mod commands;
mod database;
mod event;
mod global;
mod setup;
mod state;

use std::env;
use std::sync::Mutex;
use tauri::Manager;
use tracing::{debug, error, info, warn};

use crate::commands::user_commands;
use crate::database::get_connection_pool;
use crate::database::models::user::NewUser;
use crate::database::queries::user_query::{create_user, get_user};
use crate::setup::{setup_app, setup_logger};
use crate::state::DbState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup_logger().expect("Logger setup failed");
            setup_app().expect("Application setup failed");

            let pool = match get_connection_pool() {
                Ok(pool) => pool,
                Err(e) => {
                    error!("Failed to create database connection pool: {:?}", e);
                    return Err(e.into());
                }
            };

            let mut conn = pool.get()?;

            match get_user(&mut conn) {
                Ok(Some(_)) => {}
                Ok(None) => {
                    let result = create_user(
                        &mut conn,
                        NewUser {
                            username: String::from("john_doe"),
                            currency_api_key: None,
                        },
                    );

                    if let Err(e) = result {
                        error!("Failed to create default user: {:?}", e);
                        return Err(e.into());
                    }
                }
                Err(e) => {
                    error!("Failed to check for existing user: {:?}", e);
                    return Err(e.into());
                }
            }

            app.manage(Mutex::new(DbState::new(pool)));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            user_commands::get_user,
            user_commands::create_user,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running application");
}
