use tauri::AppHandle;

use crate::commands::{Response, ValidationError};
use crate::database::establish_connection;
use crate::database::models::account::{NewAccount, Account, UpdateAccount};
use crate::database::queries::account_query;

#[tauri::command]
pub fn get_account_by_id(app: AppHandle, account_id: i32) -> Response<Account, String> {
    let mut conn = match establish_connection(&app) {
        Ok(conn) => conn,
        Err(_) => {
            return Response::error("Failed to establish database connection", None);
        }
    };

    match account_query::get_account_by_id(&mut conn, account_id) {
        Ok(account) => Response::success("Account retrieved successfully", account),
        Err(_) => Response::error("Failed to retrieve account", None),
    }
}

#[tauri::command]
pub fn create_account(app: AppHandle, data: NewAccount) -> Response<Account, Vec<ValidationError>> {
    if let Some(errors) = validate_new_account(&data) {
        return Response::validation_error("Invalid inputs", errors);
    }

    let mut conn = match establish_connection(&app) {
        Ok(conn) => conn,
        Err(_) => {
            return Response::error("Failed to establish database connection", None);
        }
    };

    match account_query::create_account(&mut conn, data) {
        Ok(account) => Response::success("Account created successfully", Some(account)),
        Err(_) => Response::error("Failed to create account", None),
    }
}

#[tauri::command]
pub fn update_account(app: AppHandle, data: UpdateAccount) -> Response<Account, Vec<ValidationError>> {
    if let Some(errors) = validate_update_account(&data) {
        return Response::validation_error("Invalid inputs", errors);
    }

    let mut conn = match establish_connection(&app) {
        Ok(conn) => conn,
        Err(_) => {
            return Response::error("Failed to establish database connection", None);
        }
    };

    match account_query::update_account(&mut conn, data) {
        Ok(account) => Response::success("Account updated successfully", Some(account)),
        Err(_) => Response::error("Failed to update account", None),
    }
}

fn validate_new_account(data: &NewAccount) -> Option<Vec<ValidationError>> {
    let mut errors = Vec::new();

    if data.name.trim().is_empty() {
        errors.push(ValidationError::new(
            "name",
            "Name must not be empty",
        ));
    }

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}

fn validate_update_account(data: &UpdateAccount) -> Option<Vec<ValidationError>> {
    let mut errors = Vec::new();

    if data.name.trim().is_empty() {
        errors.push(ValidationError::new(
            "name",
            "Name must not be empty",
        ));
    }

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}
