pub mod enums;
pub mod models;
pub mod queries;
pub mod schema;

use crate::utils::{constants, get_app_dir_path};
use anyhow::{Context, Result};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::MigrationHarness;

fn get_db_url() -> Result<String> {
    let app_dir = get_app_dir_path().context("Failed to get application directory path")?;
    Ok(app_dir.join(constants::DATABASE_FILE).display().to_string())
}

pub fn establish_direct_connection() -> Result<SqliteConnection> {
    let db_url = get_db_url()?;
    let mut conn =
        SqliteConnection::establish(&db_url).context("Failed to establish database connection")?;

    diesel::sql_query("PRAGMA foreign_keys = ON;")
        .execute(&mut conn)
        .context("Failed to enable foreign keys")?;
    diesel::sql_query("PRAGMA busy_timeout = 5000;")
        .execute(&mut conn)
        .context("Failed to set busy timeout")?;
    diesel::sql_query("PRAGMA journal_mode = WAL;")
        .execute(&mut conn)
        .context("Failed to set journal mode to WAL")?;
    diesel::sql_query("PRAGMA cache_size = 100000;")
        .execute(&mut conn)
        .context("Failed to set cache size")?;
    diesel::sql_query("PRAGMA synchronous = NORMAL;")
        .execute(&mut conn)
        .context("Failed to set synchronous mode to NORMAL")?;
    diesel::sql_query("PRAGMA auto_vacuum = FULL;")
        .execute(&mut conn)
        .context("Failed to set auto vacuum mode to FULL")?;

    Ok(conn)
}

pub fn establish_connection() -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
    let pool = get_connection_pool()?;
    let mut conn = pool.get().context("Failed to get connection from pool")?;

    diesel::sql_query("PRAGMA foreign_keys = ON;")
        .execute(&mut conn)
        .context("Failed to enable foreign keys")?;
    diesel::sql_query("PRAGMA busy_timeout = 5000;")
        .execute(&mut conn)
        .context("Failed to set busy timeout")?;
    diesel::sql_query("PRAGMA journal_mode = WAL;")
        .execute(&mut conn)
        .context("Failed to set journal mode to WAL")?;
    diesel::sql_query("PRAGMA cache_size = 100000;")
        .execute(&mut conn)
        .context("Failed to set cache size")?;
    diesel::sql_query("PRAGMA synchronous = NORMAL;")
        .execute(&mut conn)
        .context("Failed to set synchronous mode to NORMAL")?;
    diesel::sql_query("PRAGMA auto_vacuum = FULL;")
        .execute(&mut conn)
        .context("Failed to set auto vacuum mode to FULL")?;

    Ok(conn)
}

pub fn get_connection_pool() -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db_url = get_db_url()?;
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);

    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .context("Failed to create database connection pool")
}

pub fn run_migrations() -> Result<()> {
    let mut conn = establish_direct_connection()?;
    conn.run_pending_migrations(constants::MIGRATIONS)
        .context("Failed to run database migrations")?;

    Ok(())
}

pub fn reset_all() -> Result<()> {
    let mut conn = establish_direct_connection()?;

    diesel::sql_query("PRAGMA foreign_keys = OFF;")
        .execute(&mut conn)
        .context("Failed to disable foreign keys")?;
    diesel::sql_query("DROP TABLE IF EXISTS users;")
        .execute(&mut conn)
        .context("Failed to drop users table")?;
    diesel::sql_query("DROP TABLE IF EXISTS accounts;")
        .execute(&mut conn)
        .context("Failed to drop accounts table")?;
    diesel::sql_query("DROP TABLE IF EXISTS transactions;")
        .execute(&mut conn)
        .context("Failed to drop transactions table")?;
    diesel::sql_query("DROP TABLE IF EXISTS tags;")
        .execute(&mut conn)
        .context("Failed to drop tags table")?;
    diesel::sql_query("DROP TABLE IF EXISTS limits;")
        .execute(&mut conn)
        .context("Failed to drop limits table")?;
    diesel::sql_query("DROP TABLE IF EXISTS currencies;")
        .execute(&mut conn)
        .context("Failed to drop currencies table")?;
    diesel::sql_query("PRAGMA foreign_keys = ON;")
        .execute(&mut conn)
        .context("Failed to re-enable foreign keys")?;

    run_migrations()
}
