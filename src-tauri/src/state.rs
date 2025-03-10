use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub struct DbState {
    pub conn_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DbState {
    pub fn new(conn_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { conn_pool }
    }
}
