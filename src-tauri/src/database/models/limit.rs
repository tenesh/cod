use diesel::{Queryable, Selectable};
use crate::database::enums;
use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::limits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Limit {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub interval: enums::LimitInterval,
    pub custom_interval_days: Option<i32>,
    pub amount: f32,
    pub created_at: String,
    pub updated_at: String,
}