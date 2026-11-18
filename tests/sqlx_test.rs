use sqlx::postgres::PgPoolOptions;

fn _test() -> Result<(), Box<dyn std::error::Error>> {
    let _handle = tokio::spawn(async {
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://weather_admin:12345678@localhost/weather")
            .await
            .unwrap();
    });

    Ok(())
}
