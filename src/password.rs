use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHasher};

pub async fn hash(password: String) -> Result<String, argon2::password_hash::Error> {
    tokio::task::spawn_blocking(move || {
        let hash = Argon2::default()
            .hash_password(password.as_bytes())?
            .to_string();
        Ok(hash)
    })
    .await
    .unwrap()
}

pub async fn verify_password(
    password: String,
    hash: String,
) -> Result<(), argon2::password_hash::Error> {
    tokio::task::spawn_blocking(move || {
        Argon2::default().verify_password(password.as_bytes(), hash.as_str())
    })
    .await
    .unwrap()
}
