use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{
    Connection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use tracing::{debug, error, info, warn};

use crate::database::models::account::{Account, NewAccount, UpdateAccount};
use crate::database::schema::accounts;
use crate::database::schema::accounts::dsl::*;

pub fn get_account_by_id(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    account_id: i32,
) -> Result<Option<Account>, Error> {
    match accounts
        .find(account_id)
        .select(Account::as_select())
        .first::<Account>(conn)
        .optional()
    {
        Ok(account) => Ok(account),
        Err(e) => {
            error!("Database query failed: get_account_by_id | Error: {:?}", e);
            Err(e)
        }
    }
}

pub fn create_account(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    data: NewAccount,
) -> Result<Account, Error> {
    conn.transaction(|conn| {
        match diesel::insert_into(accounts::table)
            .values(&data)
            .returning(Account::as_returning())
            .get_result(conn)
        {
            Ok(account) => Ok(account),
            Err(e) => {
                error!("Database query failed: create_account | Error: {:?}", e);
                Err(e)
            }
        }
    })
}

pub fn update_account(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    data: UpdateAccount,
) -> Result<Account, Error> {
    conn.transaction(|conn| {
        match diesel::update(&data)
            .set(&data)
            .returning(Account::as_returning())
            .get_result(conn)
        {
            Ok(account) => Ok(account),
            Err(e) => {
                error!("Database query failed: update_account | Error: {:?}", e);
                Err(e)
            }
        }
    })
}
