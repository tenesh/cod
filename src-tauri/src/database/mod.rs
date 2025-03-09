pub mod enums;
pub mod models;
pub mod queries;
pub mod schema;

use anyhow::{anyhow, Context, Result};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tracing::{info, warn, error, debug};

use crate::global::get_app_db_file_path;
use crate::state::DbState;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn get_db_url() -> Result<String> {
    let db_file = get_app_db_file_path();

    #[cfg(target_os = "windows")]
    {
        Ok(format!("sqlite:///{}", db_file.display()))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        Ok(format!("sqlite://{}", db_file.display()))
    }
}

pub fn establish_direct_connection() -> Result<SqliteConnection> {
    let db_url = get_db_url()?;
    let mut conn = SqliteConnection::establish(&db_url)?;

    info!("Successfully established direct SQLite connection");

    diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA busy_timeout = 5000;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA cache_size = 100000;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA auto_vacuum = FULL;").execute(&mut conn)?;

    Ok(conn)
}

pub fn establish_connection(
    app: &AppHandle,
) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
    let state_mutex = app.state::<Mutex<DbState>>();
    let state_lock = state_mutex
        .lock()
        .map_err(|e| anyhow!("Failed to lock database state: {:?}", e))?;
    let pool = &state_lock.conn_pool;

    let mut conn = pool.get()?;

    info!("Successfully retrieved a pooled database connection");

    diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA busy_timeout = 5000;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA cache_size = 100000;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(&mut conn)?;
    diesel::sql_query("PRAGMA auto_vacuum = FULL;").execute(&mut conn)?;

    Ok(conn)
}

pub fn get_connection_pool() -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db_url = get_db_url()?;
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)?;

    info!("Database connection pool created successfully");

    Ok(pool)
}

pub fn run_migrations() -> Result<()> {
    let mut conn = establish_direct_connection()?;
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {
            info!("Database migrations completed successfully.");
            Ok(())
        }
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(anyhow!("Failed to run database migrations"))
        }
    }
}
