use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use uuid::Uuid;
use validator::Validate;

use weather_api::encription;
use weather_api::error::*;
use weather_api::user::*;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let database_url =
        env::var("DATABASE_URL").expect(r#"Environment variable "DATABASE_URL" must be set"#);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/register", post(register))
        .route("/log_in", post(log_in))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    State(pg_pool): State<Pool<Postgres>>,
    Json(sign_in_request): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    sign_in_request.validate()?;

    let password_hash = encription::hash(sign_in_request.password)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("INSERT INTO users(id, name, email, pass_hash) VALUES ($1, $2, $3, $4)")
        .bind(Uuid::now_v7())
        .bind(sign_in_request.name)
        .bind(sign_in_request.email)
        .bind(password_hash)
        .execute(&pg_pool)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn log_in(
    State(pg_pool): State<Pool<Postgres>>,
    Json(login_request): Json<LoginRequest>,
) -> Result<StatusCode, AppError> {
    login_request.validate()?;

    let pass_hash: String = sqlx::query_scalar("SELECT pass_hash FROM users WHERE email = $1")
        .bind(login_request.email)
        .fetch_one(&pg_pool)
        .await?;

    encription::verify_password(login_request.password, pass_hash).await?;

    //sqlx::query_scalar("SELECT FROM")
    Ok(StatusCode::OK)
}
