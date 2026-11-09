use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn build_pool() -> Result<PgPool, sqlx::Error> {
    let host = std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
    let port: u16 = std::env::var("DATABASE_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5432u16);
    let username = std::env::var("DATABASE_USER").unwrap_or("postgres".to_string());
    let password = std::env::var("DATABASE_PASS").unwrap_or("password".to_string());
    let database = std::env::var("DATABASE_NAME").unwrap_or("postgres".to_string());

    let schema = std::env::var("DATABASE_SCHEMA").unwrap_or("".to_string());

    let mut conn_opts = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .password(&password)
        .database(&database);
    if !schema.is_empty() {
        conn_opts = conn_opts.options([("search_path", schema.as_str())]);
    }

    let max_life_time = if |_| {
        std::env::var("DATABASE_MAX_LIFETIME")
            .ok()
            .and_then(|d| d.parse::<u64>().ok())
            .unwrap_or(0)
            == 0
    } {
        None
    } else {
        Some(Duration::from_millis(300))
    };

    let pool_size = std::env::var("DATABASE_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    PgPoolOptions::new()
        .max_connections(pool_size)
        .acquire_timeout(Duration::from_secs(2))
        .max_lifetime(max_life_time)
        .connect_with(conn_opts)
        .await
}
