use std::env;
use std::sync::Arc;

use pasetors::claims::Claims;
use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::version4::V4;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use uuid::Uuid;
use validator::Validate;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};

use weather_api::prelude::*;

#[derive(Clone)]
struct AppState {
    pg_pool: Pool<Postgres>,
    key_pair: Arc<AsymmetricKeyPair<V4>>,
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
        key_pair: Arc::new(kp),
    };

    let app = Router::new()
        .route("/register", post(register))
        .route("/log_in", get(log_in))
        .route("/weather", get(weather))
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

    let paylaod = PASETOPayLoad {
        name: user.name,
        role: user.role.into(),
    };

    let claims = Claims::try_from(paylaod)?;
    let token = generate_public(&app_state.key_pair.secret, &claims)?;

    Ok(Json(json!({"access_token": token})))
}

async fn weather(
    State(app_state): State<AppState>,
    Json(weather_request): Json<WeatherRequest>,
) -> Result<Json<WeatherReport>, AppError> {
    let node: SQLNode = sqlx::query_as(
        "
        SELECT 
            id,
            continent,
            country,
            region,
            city,
            district
        FROM nodes
        ORDER BY (
            6371.0 * acos(
                LEAST(1.0, GREATEST(-1.0,
                    cos(radians($1)) * cos(radians(lat)) *
                    cos(radians(lon) - radians($2)) +
                    sin(radians($1)) * sin(radians(lat))
                ))
            )
        ) ASC
        LIMIT 1;
    ",
    )
    .bind(weather_request.lat)
    .bind(weather_request.lon)
    .fetch_one(&app_state.pg_pool)
    .await?;

    let logs: Vec<SQLog> = sqlx::query_as(
        "
        SELECT 
            temp,
            hum,
            air_q,
            timestamp
        FROM logs
        WHERE node_id = $1
        AND timestamp >= NOW() - INTERVAL '10 hours'
        ORDER BY timestamp DESC;
    ",
    )
    .bind(node.id)
    .fetch_all(&app_state.pg_pool)
    .await?;

    let weather_report = WeatherReport {
        location: node.into(),
        logs: logs.into_iter().map(|v|v.into()).collect()
    };

    Ok(Json(weather_report))
}
