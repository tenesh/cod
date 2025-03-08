use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub const DIRECTORY_NAME: &str = r#"netbalance"#;
pub const DATABASE_FILE: &str = r#"db.sqlite3"#;
pub const LOG_FILE: &str = r#"log.txt"#;
pub const CONFIG_FILE: &str = r#"config.json"#;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");