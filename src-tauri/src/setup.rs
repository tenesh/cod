use anyhow::Result;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use tracing::{debug, error, info, warn};

use crate::database::models::user::NewUser;
use crate::database::queries::user_query::{create_user, get_user};
use crate::database::{models, run_migrations};
use crate::global::{get_app_config_dir_path, get_app_db_file_path, get_app_log_file_path};

pub fn setup_logger() -> Result<()> {
    info!("Start logger setup");

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

    info!("Completed logger setup");

    Ok(())
}

pub fn setup_app() -> Result<()> {
    info!("Start application setup");

    let app_dir = get_app_config_dir_path();
    let db_file = get_app_db_file_path();
    let log_file = get_app_log_file_path();

    info!("Application config directory: {:?}", app_dir);
    info!("Database file: {:?}", db_file);
    info!("Log file: {:?}", log_file);

    run_migrations()?;

    info!("Database migrations completed successfully");

    info!("Completed application setup");
    Ok(())
}

pub fn setup_user(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<models::user::User> {
    match get_user(conn)? {
        Some(user) => Ok(user),
        None => {
            let new_user = create_user(
                conn,
                NewUser {
                    username: String::from("john_doe"),
                    currency_api_key: None,
                },
            )?;
            Ok(new_user)
        }
    }
}
