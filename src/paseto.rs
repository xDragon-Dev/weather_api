use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;

use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey, SymmetricKey};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{Public, public};
use pasetors::{local,Local};

#[derive(Serialize, Deserialize)]
pub enum PASETORole {
    Admin,
    Regular,
}

#[derive(Serialize, Deserialize)]
pub struct PASETOPayLoad {
    #[serde(rename="sub")]
    pub name: String,
    pub role: PASETORole,
}

impl TryFrom<PASETOPayLoad> for Claims {
    type Error = pasetors::errors::Error;

    fn try_from(value: PASETOPayLoad) -> Result<Self, Self::Error> {
        let duration = Duration::from_mins(15);
        let mut claims = Claims::new_expires_in(&duration)?;
        claims.subject(&value.name)?;
        claims.add_additional("role", json!(&value.role))?;
        
        Ok(claims)
    }
}

pub fn generate_public(secret_key: &AsymmetricSecretKey<V4>, message: &Claims) -> Result<String,pasetors::errors::Error> {
    public::sign(secret_key, message, None, None)
}

pub fn validate_public<T: serde::de::DeserializeOwned>(public_key: &AsymmetricPublicKey<V4>, token: &str) -> Result<T,pasetors::errors::Error> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public,V4>::try_from(token)?;
    let trusted_token = public::verify(public_key, &untrusted_token, &validation_rules, None, None)?;
    let payload: T = serde_json::from_str(trusted_token.payload()).map_err(|_|pasetors::errors::Error::ClaimInvalidJson)?;
    Ok(payload)
}

pub fn generate_local(secret_key: &SymmetricKey<V4>, message: &Claims) -> Result<String,pasetors::errors::Error> {
    local::encrypt(secret_key, message, None, None)
}

pub fn validate_local<T: serde::de::DeserializeOwned>(secret_key: &SymmetricKey<V4>, token: &str) -> Result<T,pasetors::errors::Error> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local,V4>::try_from(token)?;
    let trusted_token = local::decrypt(secret_key, &untrusted_token, &validation_rules, None, None)?;
    let payload: T = serde_json::from_str(trusted_token.payload()).map_err(|_|pasetors::errors::Error::ClaimInvalidJson)?;
    Ok(payload)
}