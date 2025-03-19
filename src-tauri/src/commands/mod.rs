use serde::{Deserialize, Serialize};

pub mod user_commands;
pub mod account_commands;

#[derive(Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response<T, E> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub errors: Option<E>,
    pub validation_errors: Option<Vec<ValidationError>>
}

impl<T, E> Response<T, E> {
    pub fn success(message: &str, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data,
            errors: None,
            validation_errors: None,
        }
    }

    pub fn error(message: &str, errors: Option<E>) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            errors,
            validation_errors: None,
        }
    }

    pub fn validation_error(message: &str, validation_errors: Vec<ValidationError>) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            errors: None,
            validation_errors: Some(validation_errors),
        }
    }
}
