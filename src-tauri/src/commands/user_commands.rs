use tauri::AppHandle;

use crate::commands::{Response, ValidationError};
use crate::database::establish_connection;
use crate::database::models::user::{NewUser, User};
use crate::database::queries::user_query;

#[tauri::command]
pub fn get_user(app: AppHandle) -> Response<User, String> {
    let mut conn = match establish_connection(&app) {
        Ok(conn) => conn,
        Err(_) => {
            return Response::error("Failed to establish database connection", None);
        }
    };

    match user_query::get_user(&mut conn) {
        Ok(user) => Response::success("User retrieved successfully", user),
        Err(_) => Response::error("Failed to retrieve user", None),
    }
}

#[tauri::command]
pub fn create_user(app: AppHandle, new_user: NewUser) -> Response<User, Vec<ValidationError>> {
    if let Some(errors) = validate_user(&new_user) {
        return Response::validation_error("Invalid inputs", errors);
    }

    let mut conn = match establish_connection(&app) {
        Ok(conn) => conn,
        Err(_) => {
            return Response::error("Failed to establish database connection", None);
        }
    };

    match user_query::create_user(&mut conn, new_user) {
        Ok(user) => Response::success("User created successfully", Some(user)),
        Err(_) => Response::error("Failed to create user", None),
    }
}

fn validate_user(new_user: &NewUser) -> Option<Vec<ValidationError>> {
    let mut errors = Vec::new();

    if new_user.username.len() < 4 {
        errors.push(ValidationError::new(
            "username",
            "Username must be at least 4 characters long",
        ));
    }

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}
