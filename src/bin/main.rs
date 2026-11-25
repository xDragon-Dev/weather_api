use std::env;
use std::sync::Arc;

use pasetors::claims::Claims;
use uuid::Uuid;
use validator::Validate;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use weather_api::password;
use weather_api::error::*;
use weather_api::http_types::*;
use weather_api::paseto::*;
use weather_api::sql_types;
use weather_api::sql_types::Role;
use weather_api::sql_types::*;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::version4::V4;


#[derive(Clone)]
struct AppState {
    pg_pool: Pool<Postgres>,
    key_pair: Arc<AsymmetricKeyPair<V4>>
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let database_url =
        env::var("DATABASE_URL").expect(r#"Environment variable "DATABASE_URL" must be set"#);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let kp = AsymmetricKeyPair::generate()?;

    let app_state = AppState {
        pg_pool: pool,
        key_pair: Arc::new(kp)
    };

    let app = Router::new()
        .route("/register", post(register))
        .route("/log_in", post(log_in))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    State(app_state): State<AppState>,
    Json(sign_in_request): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    sign_in_request.validate()?;

    let password_hash = password::hash(sign_in_request.password)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("INSERT INTO users(id, name, email, pass_hash, role) VALUES ($1, $2, $3, $4, $5)")
        .bind(Uuid::now_v7())
        .bind(sign_in_request.name)
        .bind(sign_in_request.email)
        .bind(password_hash)
        .bind(sql_types::Role::Regular)
        .execute(&app_state.pg_pool)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn log_in(
    State(app_state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<String, AppError> {
    login_request.validate()?;

    let (pass_hash, name, role): (String, String, Role) = sqlx::query_as("SELECT pass_hash, name FROM users WHERE email = $1")
        .bind(login_request.email)
        .fetch_one(&app_state.pg_pool)
        .await?;

    password::verify_password(login_request.password, pass_hash).await?;
    
    let claims: PayLoad = PayLoad {
        name: name, role: role.into()
    };

    let claims = Claims::try_from(claims)?;
    let token = generate_public(&app_state.key_pair.secret, &claims)?;
    
    Ok(token)
}
