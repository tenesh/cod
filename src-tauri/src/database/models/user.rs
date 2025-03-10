use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::database::schema;

#[derive(Queryable, Selectable, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub currency_api_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub username: String,
    pub currency_api_key: Option<String>,
}