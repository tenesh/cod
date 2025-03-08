use crate::api::ApiResponse;
use crate::database::models::user::{NewUser, User};
use crate::database::queries::user_query;
use diesel::SqliteConnection;

pub fn get_user(conn: &mut SqliteConnection, user_id: i32) -> ApiResponse<User> {
    match user_query::get_user_by_id(conn, user_id) {
        Ok(user) => ApiResponse::success("User retrieved successfully.", Option::from(user)),
        Err(_) => ApiResponse::error("User not found."),
    }
}

pub fn create_user(conn: &mut SqliteConnection, new_user: NewUser) -> ApiResponse<User> {
    match user_query::create_user(conn, new_user) {
        Ok(user) => ApiResponse::success("User created successfully.", Option::from(user)),
        Err(_) => ApiResponse::error("Failed to create user."),
    }
}

pub fn validate_username(input: &str) -> Result<()> {
    if input.len() < 4 {
        return Err(UserValidationError::InvalidUsernameError);
    }
    Ok(())
}
