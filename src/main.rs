mod data_types;
mod sql_types;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let database_url =
        env::var("DATABASE_URL").expect(r#"Environment variable "DATABASE_URL" must be set"#);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new().route("/login", get(log_in)).with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn log_in(State(pg_pool): State<Pool<Postgres>>) {
    let value: (i32,) = sqlx::query_as("SELECT $1")
        .bind(10_i32)
        .fetch_one(&pg_pool)
        .await
        .unwrap();
    println!("value: {}", value.0);
}
