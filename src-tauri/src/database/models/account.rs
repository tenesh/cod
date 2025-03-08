use diesel::{Queryable, Selectable};
use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub currency_id: i32,
    pub created_at: String,
    pub updated_at: String,
}