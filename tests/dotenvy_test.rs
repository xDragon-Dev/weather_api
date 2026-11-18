use std::env;
use std::error::Error;

#[test]
fn loads_enviroment_variable() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let value = env::var("VARIABLE")?;
    println!("{value}");
    Ok(())
}

#[test]
fn fails_loading_non_existent_env_var() {
    dotenvy::dotenv().unwrap();
    let result = env::var("NON_EXIXTENT");
    assert!(result.is_err())
}

#[test]
fn env_var_matches_correct_value() {
    dotenvy::dotenv().unwrap();
    let value = env::var("VARIABLE").unwrap();
    assert_eq!(value, "value");
}
