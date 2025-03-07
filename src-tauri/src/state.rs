use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub struct AppState {
    pub init: bool,
    pub conn_pool: Option<Pool<ConnectionManager<SqliteConnection>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            conn_pool: None,
            init: false,
        }
    }

    pub fn set_pool(&mut self, pool: Pool<ConnectionManager<SqliteConnection>>) {
        self.conn_pool = Some(pool);
    }

    pub fn set_init(&mut self, init: bool) {
        self.init = init;
    }
}
