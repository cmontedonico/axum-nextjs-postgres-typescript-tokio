use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is required"))?;

        Ok(Self {
            url,
            max_connections: 20,
            min_connections: 5,
            connect_timeout: Duration::from_secs(10),
        })
    }

    pub async fn create_pool(&self) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.connect_timeout)
            .connect(&self.url)
            .await?;

        Ok(pool)
    }
}