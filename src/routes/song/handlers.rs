use actix_web::{get, post, web};
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::data;
use crate::extractor::auth::Auth;
use crate::types::{IntoResponseResult, ResponseResult};

use super::model::{request, response};

#[post("/local")]
pub async fn add_local_song(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    auth: Auth,
    body: web::Json<request::payload::AddLocalSong>,
) -> ResponseResult<()> {
    let pool = pool.get_ref();
    let body = body.into_inner();
    let config = config.get_ref();

    data::song::add_local_to_history(
        pool,
        config,
        auth.user_id,
        body.title,
        body.artist,
        body.uri,
        body.provider,
        body.location.lat,
        body.location.lon,
        body.world_id,
    ).await?;

    ().into_response()
}

#[post("/existing")]
pub async fn add_existing_song(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    auth: Auth,
    body: web::Json<request::payload::AddExistingSong>,
) -> ResponseResult<()> {
    let pool = pool.get_ref();
    let body = body.into_inner();
    let config = config.get_ref();

    data::song::add_existing_to_history(
        pool,
        config,
        auth.user_id,
        body.song_id,
        body.location.lat,
        body.location.lon,
        body.world_id,
    ).await?;

    ().into_response()
}

/// Gets a list of song from the database based on the
/// following criteria
///
/// - Location (`latitude`, `longitude`)
/// - Provider (`provider`) [e.g "spotify"]
/// - Optionally World (`world_id`)
///
/// This will search for stations nearby with song
/// pertaining to the world mentioned (if so)
///
/// It will find stations within a pre-determined radius
/// and optionally also ensure that those song belong to
/// the world mentioned. It will also return the play count
/// of each song found
#[get("/nearby")]
pub async fn get_songs_nearby(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    _: Auth,
    query: web::Query<request::query::GetNearby>,
) -> ResponseResult<Vec<response::song::NearbySong>> {
    let query = query.into_inner();
    let pool = pool.get_ref();
    let config = config.get_ref();

    // Database call
    let results = data::song::get_nearby(
        pool,
        config,
        query.lat,
        query.lon,
        query.provider,
        query.world_id,
    ).await?;

    let results: Vec<response::song::NearbySong> = results.into_iter()
        .map(Into::into)
        .collect();

    results.into_response()
}