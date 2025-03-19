use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{Connection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use tracing::{debug, error, info, warn};

use crate::database::models::user::{NewUser, UpdateUser, User};
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
    data: NewUser,
) -> Result<User, Error> {

    conn.transaction(|conn| {
        match diesel::insert_into(users::table)
            .values(&data)
            .returning(User::as_returning())
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Database query failed: create_user | Error: {:?}", e);
                Err(e)
            }
        }
    })
}

pub fn update_user(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    data: UpdateUser,
) -> Result<User, Error> {

    conn.transaction(|conn| {
        match diesel::update(&data)
            .set(&data)
            .returning(User::as_returning())
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Database query failed: update_user | Error: {:?}", e);
                Err(e)
            }
        }
    })
}
