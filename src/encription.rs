use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHasher};
use std::sync::LazyLock;

static ARGON2: LazyLock<Argon2> = LazyLock::new(|| Argon2::default());

pub fn hash(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let hash = (*ARGON2).hash_password(password)?.to_string();
    Ok(hash)
}

pub fn _verify_password(password: &[u8], hash: &str) -> Result<(), argon2::password_hash::Error> {
    (*ARGON2).verify_password(password, hash)
}
