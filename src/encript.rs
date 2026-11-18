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

#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher};

    #[test]
    fn hashing_and_verification() -> Result<(), argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let password = String::from("AnyPassword");

        //We save this password in the database
        let hashed_password = argon2.hash_password(password.as_bytes())?.to_string();

        //We create a new PasswordHash object from a hashed password string
        //Requiered becausse we save our password as a string in database
        let parsed_hash = PasswordHash::new(&hashed_password)?;

        argon2.verify_password(password.as_bytes(), &parsed_hash)
    }
}
