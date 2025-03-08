use diesel::{Insertable, Queryable, Selectable};
use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub currency_api_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub username: String,
    pub currency_api_key: Option<String>,
}