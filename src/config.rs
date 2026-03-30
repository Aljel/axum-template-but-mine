use sqlx::postgres::PgPoolOptions;
use std::env;

pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub secret_refresh_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect(".env not loaded"),
            secret_key: env::var("JWT_SECRET").expect(".env not loaded"),
            secret_refresh_key: env::var("JWT_SECRET_REFRESH").expect(".env not loaded"),
        }
    }
}

pub async fn get_db_pool(database_url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        .connect(database_url)
        .await
        .unwrap()
}
