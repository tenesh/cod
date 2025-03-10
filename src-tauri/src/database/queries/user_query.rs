use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use tracing::{debug, error, info, warn};

use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;
use crate::database::schema::users::dsl::*;

pub fn get_user(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Option<User>, Error> {
    match users
        .select(User::as_select())
        .first::<User>(conn)
        .optional()
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Database query failed: get_user | Error: {:?}", e);
            Err(e)
        }
    }
}

pub fn create_user(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    new_user: NewUser,
) -> Result<User, Error> {
    match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Database query failed: create_user | Error: {:?}", e);
            Err(e)
        }
    }
}
