use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub struct DbState {
    pub conn_pool: Pool<ConnectionManager<SqliteConnection>>,
}

pub struct SetupState {
    pub backend_done: bool,
}

pub struct UserState {
    pub id: Option<i32>,
    pub username: Option<String>,
}

impl DbState {
    pub fn new(conn_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { conn_pool }
    }
}

impl SetupState {
    pub fn new(backend_done: bool) -> Self {
        Self { backend_done }
    }
}

impl UserState {
    pub fn new(id: i32, username: String) -> Self {
        Self {
            id: Some(id),
            username: Some(username),
        }
    }
}
