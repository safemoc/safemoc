use dotenvy::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn build_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok().unwrap();
    let host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let port: u16 = std::env::var("DB_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5432u16);
    let username = std::env::var("DB_USER").unwrap_or("postgres".to_string());
    let password = std::env::var("DB_PASS").unwrap_or("password".to_string());
    let database = std::env::var("DB_NAME").unwrap_or("postgres".to_string());

    let schema = std::env::var("DB_SCHEMA").unwrap_or("".to_string());

    let mut conn_opts = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .password(&password)
        .database(&database);
    if !schema.is_empty() {
        conn_opts = conn_opts.options([("search_path", schema.as_str())]);
    }
    let mut max_life_time = std::env::var("DB_MAX_LIFETIME")
        .ok()
        .and_then(|d| d.parse::<u64>().ok())
        .unwrap_or(0);
    let max_lifetime = if max_life_time == 0 {
        None
    } else {
        Some(Duration::from_millis(max_life_time))
    };

    let acquire_timeout = std::env::var("DB_ACCUMILATE_TIMEOUT")
        .ok()
        .and_then(|d| d.parse::<u64>().ok())
        .unwrap_or(0);

    let pool_size = std::env::var("DB_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    PgPoolOptions::new()
        .max_connections(pool_size)
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .max_lifetime(max_lifetime)
        .connect_with(conn_opts)
        .await
}
