use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;

use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{
    AsymmetricKeyPair, AsymmetricPublicKey, AsymmetricSecretKey, Generate, SymmetricKey,
};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{Local, local};
use pasetors::{Public, public};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum Role {
    Admin,
    Regular,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct PayLoad {
    #[serde(rename = "sub")]
    name: String,
    role: Role,
}

impl From<PayLoad> for Claims {
    fn from(value: PayLoad) -> Self {
        let mut claims = Claims::new_expires_in(&Duration::from_mins(15)).unwrap();
        claims.subject(&value.name).unwrap();
        claims.add_additional("role", json!(&value.role)).unwrap();

        claims
    }
}

#[test]
fn generate_and_validate_public() {
    let payload = PayLoad {
        name: String::from("Juan"),
        role: Role::Admin,
    };

    let kp = AsymmetricKeyPair::<V4>::generate().unwrap();

    let claims = Claims::from(payload.clone());
    let public_token = generate_public(&kp.secret, &claims).unwrap();
    let payload_output: PayLoad = validate_public(&kp.public, &public_token).unwrap();

    assert_eq!(payload, payload_output);
}

#[test]
fn generate_and_validate_local() {
    let payload = PayLoad {
        name: String::from("Juan"),
        role: Role::Admin,
    };

    let sk = SymmetricKey::generate().unwrap();

    let claims = Claims::from(payload.clone());
    let local_token = generate_local(&sk, &claims).unwrap();
    let payload_output: PayLoad = validate_local(&sk, &local_token).unwrap();

    assert_eq!(payload, payload_output);
}

fn generate_public(
    secret_key: &AsymmetricSecretKey<V4>,
    message: &Claims,
) -> Result<String, pasetors::errors::Error> {
    public::sign(secret_key, message, None, None)
}

fn validate_public<T: serde::de::DeserializeOwned>(
    public_key: &AsymmetricPublicKey<V4>,
    token: &str,
) -> Result<T, pasetors::errors::Error> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(token)?;
    let trusted_token =
        public::verify(public_key, &untrusted_token, &validation_rules, None, None)?;
    let payload: T = serde_json::from_str(trusted_token.payload())
        .map_err(|_| pasetors::errors::Error::ClaimInvalidJson)?;
    Ok(payload)
}

fn generate_local(
    secret_key: &SymmetricKey<V4>,
    message: &Claims,
) -> Result<String, pasetors::errors::Error> {
    local::encrypt(secret_key, message, None, None)
}

fn validate_local<T: serde::de::DeserializeOwned>(
    secret_key: &SymmetricKey<V4>,
    token: &str,
) -> Result<T, pasetors::errors::Error> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(token)?;
    let trusted_token =
        local::decrypt(secret_key, &untrusted_token, &validation_rules, None, None)?;
    let payload: T = serde_json::from_str(trusted_token.payload())
        .map_err(|_| pasetors::errors::Error::ClaimInvalidJson)?;
    Ok(payload)
}
