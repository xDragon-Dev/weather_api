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

#[test]
fn verification_fails_on_incorrect_password() {
    let argon2 = Argon2::default();
    let original_password = String::from("CorrectPassword");

    //We save this password in database as a string
    let hashed_password = argon2
        .hash_password(original_password.as_bytes())
        .unwrap()
        .to_string();

    //We recieve an incorrect password from a login attempt
    let recieved_password = String::from("IncorrectPassword");

    //We create a new PasswordHash object from a hashed password string
    //Requiered becausse we save our password as a string in database
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();

    assert!(
        argon2
            .verify_password(recieved_password.as_bytes(), &parsed_hash)
            .is_err()
    )
}
