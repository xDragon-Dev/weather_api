use std::env;
use std::sync::Arc;

use axum::routing::get;
use pasetors::claims::Claims;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use weather_api::prelude::*;

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
        .await
        .expect("Pstgresql connection could not be stablished");

    let kp = AsymmetricKeyPair::generate()?;

    let app_state = AppState {
        pg_pool: pool,
        key_pair: Arc::new(kp)
    };

    let app = Router::new()
        .route("/register", post(register))
        .route("/log_in", get(log_in))
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

    let password_hash = hash(sign_in_request.password)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("INSERT INTO users(id, name, email, pass_hash, role) VALUES ($1, $2, $3, $4, $5)")
        .bind(Uuid::now_v7())
        .bind(sign_in_request.name)
        .bind(sign_in_request.email)
        .bind(password_hash)
        .bind(SQLRole::Regular)
        .execute(&app_state.pg_pool)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn log_in(
    State(app_state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    login_request.validate()?;

    let user: SQLUser = sqlx::query_as("SELECT name, pass_hash, role FROM users WHERE email = $1")
        .bind(login_request.email)
        .fetch_one(&app_state.pg_pool)
        .await?;

    verify_password(login_request.password, user.pass_hash).await?;
    
    let claims: PASETOPayLoad = PASETOPayLoad {
        name: user.name, role: user.role.into()
    };

    let claims = Claims::try_from(claims)?;
    let token = generate_public(&app_state.key_pair.secret, &claims)?;
    
    Ok(Json(json!({"access_token": token})))
}
