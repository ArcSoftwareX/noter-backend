use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("An internal error occured")]
    Internal,

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("User with email '{0}' already exists")]
    UserAlreadyExists(String),

    #[error("Failed to validate input")]
    ValidationError(Vec<(String, String)>),

    #[error("You are not logged in")]
    LoggedOut,

    #[error("Note does not exist")]
    NoteDoesNotExist,

    #[error("Invalid avatar size: {0}")]
    InvalidAvatarSize(u64),

    #[error("Filetype '{0}' is not supported")]
    InvalidFileType(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_)
            | AppError::UserAlreadyExists(_)
            | AppError::InvalidCredentials
            | AppError::LoggedOut
            | AppError::NoteDoesNotExist
            | AppError::InvalidAvatarSize(_)
            | AppError::InvalidFileType(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let json = match self {
            AppError::ValidationError(errors) => json!({ "error": errors }),
            _ => json!({ "error": self.to_string() }),
        };

        HttpResponse::build(self.status_code()).json(json)
    }
}

pub type AppResult<T> = Result<T, AppError>;
