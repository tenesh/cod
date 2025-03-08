use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;
use crate::database::schema::users::dsl::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> QueryResult<User> {
    users.find(user_id).select(User::as_select()).first::<User>(conn)
}

pub fn create_user(conn: &mut SqliteConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}
