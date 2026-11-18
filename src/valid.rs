use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,29}$").unwrap());

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignInRequest {
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
