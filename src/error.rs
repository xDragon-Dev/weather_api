use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Validation(validator::ValidationErrors),
    Database(sqlx::Error),
    UserAlreadyExists,
    Internal(String),
}

// Esto le dice a Axum cómo convertir tu error en una respuesta HTTP
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Validation(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists".to_string())
            }
            AppError::Database(err) => {
                // Logueas el error real internamente, pero no lo expones al cliente
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Internal(msg) => {
                eprintln!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Implementaciones de `From` para poder usar el operador `?` directamente
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
