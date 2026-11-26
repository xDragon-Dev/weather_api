use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use time::OffsetDateTime;
use validator::{Validate, ValidationError};

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,29}$").unwrap());

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(regex(path = *USERNAME_REGEX, message = "Username must be between 3 and 30 alphanumeric characters, starting with an alphabetic character"))]
    pub name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(custom(function = "strong_password"))]
    pub password: String,
    #[validate(must_match(other = password, message = "Passwords do not match"))]
    pub confirm_password: String,
}

fn strong_password(password: &str) -> Result<(), ValidationError> {
    if !(8..=64).contains(&password.chars().count()) {
        return Err(ValidationError::new("weak_password")
            .with_message("Password must be between 8 and 64 characters long".into()));
    }

    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_numeric = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| "!@#$%^&*_-+=".contains(c));

    if !has_lowercase || !has_uppercase || !has_numeric || !has_special {
        return Err(ValidationError::new("weak_password").with_message(
            "Password must contain uppercase, lowercase, numeric and special characters".into(),
        ));
    }

    if password
        .chars()
        .any(|c| !c.is_alphanumeric() && !"!@#$%^&*_-+=".contains(c))
    {
        return Err(ValidationError::new("invalid_characters")
            .with_message("Allowed special characters are: !@#$%^&*_-+=".into()));
    }
    Ok(())
}

#[derive(Validate, Serialize, Deserialize)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherRequest {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize,Deserialize)]
pub struct Log {
    pub temp: f32,
    pub hum: f32,
    pub air_q: f32,
    pub timestamp: OffsetDateTime,
}

pub type Logs = Vec<Log>;

#[derive(Serialize,Deserialize)]
pub struct Location {
    pub continent: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub district: String,
}

#[derive(Serialize,Deserialize)]
pub struct WeatherReport {
    pub location: Location,
    pub logs: Logs,
}

pub use error::AppError;

pub mod error {
    use axum::response::{IntoResponse, Response};
    use axum::{Json, http::StatusCode};
    use serde_json::json;

    impl From<validator::ValidationErrors> for AppError {
        fn from(err: validator::ValidationErrors) -> Self {
            AppError::Validation(err)
        }
    }

    impl From<sqlx::Error> for AppError {
        fn from(err: sqlx::Error) -> Self {
            if let sqlx::Error::Database(database_err) = &err {
                if database_err.is_unique_violation() {
                    return match database_err.constraint() {
                        Some("users_name_key") => AppError::UserAlreadyExists,
                        Some("users_email_key") => AppError::EmailAlreadyInUse,
                        _ => AppError::UserAlreadyExists,
                    };
                }
            }
            AppError::Database(err)
        }
    }

    impl From<argon2::password_hash::Error> for AppError {
        fn from(_value: argon2::password_hash::Error) -> Self {
            AppError::Authentication
        }
    }

    impl From<pasetors::errors::Error> for AppError {
        fn from(_value: pasetors::errors::Error) -> Self {
            AppError::Authorization
        }
    }

    pub enum AppError {
        Database(sqlx::Error),
        UserAlreadyExists,
        EmailAlreadyInUse,
        Validation(validator::ValidationErrors),
        Authentication,
        Authorization,
        Internal(String),
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
}
