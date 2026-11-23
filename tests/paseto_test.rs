use serde::{Deserialize, Serialize};
use serde_json::json;

use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{Public, public};

use std::sync::LazyLock;

static KP: LazyLock<AsymmetricKeyPair<V4>> =
    LazyLock::new(|| AsymmetricKeyPair::<V4>::generate().unwrap());

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum UserKind {
    Admin,
    Regular,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PayLoad {
    username: String,
    user_kind: UserKind,
}

#[test]
fn generation_and_validation() {
    let payload = PayLoad {
        username: String::from("Juan"),
        user_kind: UserKind::Admin,
    };

    let mut claims = Claims::new().unwrap();
    claims.subject(&payload.username).unwrap();

    claims
        .add_additional("role", json!(&payload.user_kind))
        .unwrap();

    let public_token = public::sign(&KP.secret, &claims, None, None).unwrap();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(&public_token).unwrap();
    let trusted_token =
        public::verify(&KP.public, &untrusted_token, &validation_rules, None, None).unwrap();

    let claims = trusted_token.payload_claims().unwrap();
    let username: String =
        serde_json::from_value(claims.get_claim("sub").unwrap().clone()).unwrap();
    let user_kind: UserKind =
        serde_json::from_value(claims.get_claim("role").unwrap().clone()).unwrap();

    let payload_output = PayLoad {
        username: username,
        user_kind: user_kind,
    };

    assert_eq!(payload, payload_output);
}

/*
let kp = AsymmetricKeyPair::<V4>::generate().unwrap();

//No sé que hacer con estas, si debería guardarlas en una variable estatica en un oncecell, o generarlas en un lazylock
// O si debería guardarlas en algun programa en el sistema operativo
// O si en la etapa de desarrollo está bien tenerlas en en .env
// O si debería generarlas a través de un programa externo, ashuda
let secret_key = kp.secret;
let public_key = kp.public;
*/
