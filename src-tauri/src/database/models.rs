use diesel::prelude::*;
use super::enums;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub decimal_digits: f64,
    pub rounding: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub party: String,
    pub description: Option<String>,
    pub color: String,
    pub currency_id: i32,
    pub conversion_rate: f64,
    pub amount: f64,
    pub category: enums::TransactionCategory,
    pub medium: enums::TransactionMedium,
    pub status: enums::TransactionStatus,
    pub account_id: i32,
    pub transacted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::limits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Limit {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub interval: enums::LimitInterval,
    pub custom_interval_days: Option<i32>,
    pub amount: f64,
    pub transacted_at: String,
    pub created_at: String,
    pub updated_at: String,
}