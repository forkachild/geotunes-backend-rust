use actix_web::{get, post, web};
use sqlx::PgPool;

use crate::data::world;
use crate::extractor::auth::Auth;
use crate::routes::world::model::request::payload::create::CreateWorld;
use crate::routes::world::model::request::query::get::GetWorlds;
use crate::routes::world::model::response::world::WorldResponse;
use crate::types::{IntoResponseResult, ResponseResult};
use crate::config::AppConfig;

#[post("")]
pub async fn create_world(
    pool: web::Data<PgPool>,
    _: Auth,
    data: web::Json<CreateWorld>,
) -> ResponseResult<WorldResponse> {
    let pool = pool.get_ref();
    let data = data.into_inner();

    let name = data.name;
    let id = world::add(pool, name.clone()).await?;

    WorldResponse::new(id, name).into_response()
}

#[get("")]
pub async fn get_worlds(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    _: Auth,
    query: web::Query<GetWorlds>,
) -> ResponseResult<Vec<WorldResponse>> {
    let pool = pool.get_ref();
    let query = query.into_inner();

    let name = query.name;
    let skip = query.skip.unwrap_or(config.default_skip);
    let limit = query.limit.unwrap_or(config.default_limit);

    let results = if let Some(name) = name {
        world::search_paginated(pool, name, skip, limit).await?
    } else {
        world::get_paginated(pool, skip, limit).await?
    };

    let results: Vec<WorldResponse> = results.into_iter().map(Into::into).collect();

    results.into_response()
}