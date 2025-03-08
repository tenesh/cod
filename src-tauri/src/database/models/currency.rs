use diesel::{Queryable, Selectable};
use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::currencies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub decimal_digits: f32,
    pub rounding: f32,
    pub created_at: String,
    pub updated_at: String,
}