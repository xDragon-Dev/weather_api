use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use uuid::Uuid;
use validator::Validate;

use weather_api::encription;
use weather_api::error::*;
use weather_api::validation::*;

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
        .route("/sign_in", post(register))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    State(pg_pool): State<Pool<Postgres>>,
    Json(sign_in_request): Json<SignInRequest>,
) -> Result<StatusCode, AppError> {
    sign_in_request.validate()?;

    let password_hash = encription::hash(sign_in_request.password)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?; // Error de tu función hash

    // 4. Inserta en la BD
    sqlx::query("INSERT INTO users(id, name, email, pass_hash) VALUES ($1, $2, $3, $4)")
        .bind(Uuid::now_v7())
        .bind(sign_in_request.name)
        .bind(sign_in_request.email)
        .bind(password_hash)
        .execute(&pg_pool)
        .await?;

    Ok(StatusCode::CREATED)
}
