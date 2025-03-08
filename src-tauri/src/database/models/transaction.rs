use diesel::{Queryable, Selectable};
use crate::database::enums;
use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub party: String,
    pub description: Option<String>,
    pub currency_id: i32,
    pub conversion_rate: f32,
    pub amount: f32,
    pub category: enums::TransactionCategory,
    pub medium: enums::TransactionMedium,
    pub status: enums::TransactionStatus,
    pub account_id: i32,
    pub transacted_at: String,
    pub created_at: String,
    pub updated_at: String,
}