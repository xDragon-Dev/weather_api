use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::test]
async fn database_working() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().unwrap();
    let database_url =
        env::var("DATABASE_URL").expect(r#"Environment variable "DATABASE_URL" must be set"#);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let value: bool = sqlx::query_scalar("SELECT $1")
        .bind(true)
        .fetch_one(&pool)
        .await?;
    assert!(value);

    Ok(())
}
