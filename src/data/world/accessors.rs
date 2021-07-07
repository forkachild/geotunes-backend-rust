use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::data::world::models::DBWorld;
use crate::types::ManagedResult;

pub async fn add(
    pool: &PgPool,
    name: String,
) -> ManagedResult<Uuid> {
    let id = sqlx::query(include_str!("sql/insert_one_world_ret_id.sql"))
        .bind(&name)
        .fetch_one(pool)
        .await?
        .get::<Uuid, _>("id");

    Ok(id)
}

pub async fn get_paginated(
    pool: &PgPool,
    skip: u32,
    limit: u32,
) -> ManagedResult<Vec<DBWorld>> {
    let results = sqlx::query_as::<_, DBWorld>(include_str!("sql/get_all_paginated.sql"))
        .bind(&skip)
        .bind(&limit)
        .fetch_all(pool)
        .await?;

    Ok(results)
}

pub async fn search_paginated(
    pool: &PgPool,
    name: String,
    skip: u32,
    limit: u32,
) -> ManagedResult<Vec<DBWorld>> {
    let results = sqlx::query_as::<_, DBWorld>(include_str!("sql/get_all_like_name_paginated.sql"))
        .bind(&name)
        .bind(&skip)
        .bind(&limit)
        .fetch_all(pool)
        .await?;

    Ok(results)
}