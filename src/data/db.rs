use sqlx::{Executor, PgPool};
use sqlx::postgres::PgPoolOptions;

use crate::config::AppConfig;
use crate::types::ManagedResult;

#[allow(dead_code)]
pub async fn create_pool(config: &AppConfig) -> ManagedResult<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(config.database_max_conn)
        .connect(&config.database_url)
        .await?)
}

#[allow(dead_code)]
pub async fn create_schema(pool: &PgPool) -> ManagedResult<()> {
    let _ = pool.execute(include_str!("./migration/up.sql")).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn destroy_schema(pool: &PgPool) -> ManagedResult<()> {
    let _ = pool.execute(include_str!("./migration/down.sql")).await?;
    Ok(())
}