use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

pub enum AppError {
    Database(sqlx::Error),
    Validation(validator::ValidationErrors),
    Authentication,
    Authorization,
    Internal(String),
    UserAlreadyExists,
    EmailAlreadyInUse,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::Authentication => (
                StatusCode::UNAUTHORIZED,
                String::from("Missing credencials to perform this operation"),
            ),
            AppError::Authorization => (
                StatusCode::UNAUTHORIZED,
                String::from("You may not have enough permissions"),
            ),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::UserAlreadyExists => {
                (StatusCode::CONFLICT, String::from("User already exists"))
            }
            AppError::EmailAlreadyInUse => (
                StatusCode::CONFLICT,
                String::from("Email is already in use"),
            ),
        };

        (status_code, Json(json! ({"message": error_message}))).into_response()
    }
}
