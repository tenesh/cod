use crate::database::schema;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Clone, Serialize, Deserialize)]
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::accounts)]
pub struct NewAccount {
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub currency_id: i32,
}

#[derive(AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = schema::accounts)]
pub struct UpdateAccount {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub currency_id: Option<i32>,
}
