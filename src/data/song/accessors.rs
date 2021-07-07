use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::data::song::models::DBNearbySong;
use crate::errors::ManagedError;
use crate::types::ManagedResult;

pub async fn add_local_to_history(
    pool: &PgPool,
    config: &AppConfig,
    user_id: Uuid,
    title: String,
    artist: String,
    uri: String,
    provider: String,
    lat: f64,
    lon: f64,
    world_id: Option<Uuid>,
) -> ManagedResult<()> {
    // Find if a song exists with the same uri, provider and/or world_id
    // or create one

    // Find nearest station around (lat, lon) within given radius
    // If not found create a station at (lat, lon)

    // Add the song to the song_history against that station

    let mut trx = pool.begin().await?;

    // Get the id of the song to be added
    let song_id = {
        // Find if there already exists a song with the uri & provider
        let result = sqlx::query(include_str!("sql/get_by_uri_provider.sql"))
            .bind(&uri)
            .bind(&provider)
            .fetch_optional(&mut trx)
            .await?;

        if let Some(result) = result {
            result.get::<Uuid, _>("song_id")
        } else {
            let song_id = sqlx::query(include_str!("sql/insert_one_song_ret_id.sql"))
                .bind(&title)
                .bind(&artist)
                .fetch_one(&mut trx)
                .await?
                .get::<Uuid, _>("id");

            let _ = sqlx::query(include_str!("sql/insert_one_song_link.sql"))
                .bind(&song_id)
                .bind(&uri)
                .bind(&provider)
                .execute(&mut trx)
                .await?;

            song_id
        }
    };

    let _ = add_to_history(
        &mut trx,
        config,
        &user_id,
        &song_id,
        &lat,
        &lon,
        &world_id,
    ).await?;

    let _ = trx.commit().await?;

    Ok(())
}

pub async fn add_existing_to_history(
    pool: &PgPool,
    config: &AppConfig,
    user_id: Uuid,
    song_id: Uuid,
    lat: f64,
    lon: f64,
    world_id: Option<Uuid>,
) -> ManagedResult<()> {
    let mut trx = pool.begin().await?;

    let exists = sqlx::query(include_str!("sql/check_exists_by_id.sql"))
        .bind(&song_id)
        .fetch_one(&mut trx)
        .await?
        .get::<bool, _>("song_exists");

    if !exists {
        return Err(ManagedError::DoesNotExist);
    }

    let _ = add_to_history(&mut trx, config, &user_id, &song_id, &lat, &lon, &world_id).await?;
    let _ = trx.commit().await?;

    Ok(())
}

pub async fn get_nearby(
    pool: &PgPool,
    config: &AppConfig,
    lat: f64,
    lon: f64,
    provider: String,
    world_id: Option<Uuid>,
) -> ManagedResult<Vec<DBNearbySong>> {
    let builder = if let Some(world_id) = world_id {
        sqlx::query_as::<_, DBNearbySong>(include_str!("sql/get_nearby_in_world.sql"))
            .bind(world_id)
    } else {
        sqlx::query_as::<_, DBNearbySong>(include_str!("sql/get_nearby.sql"))
    };

    Ok(
        builder
            .bind(lat)
            .bind(lon)
            .bind(config.search_radius)
            .bind(provider)
            .fetch_all(pool)
            .await?
    )
}

async fn add_to_history<'a>(
    exec: &mut Transaction<'a, Postgres>,
    config: &AppConfig,
    user_id: &Uuid,
    song_id: &Uuid,
    lat: &f64,
    lon: &f64,
    world_id: &Option<Uuid>,
) -> ManagedResult<()> {

    // Now carefully ensure that there is a station nearby to add the song to
    let station_id = {
        let result = sqlx::query(include_str!("sql/get_nearby_station_id.sql"))
            .bind(lat)
            .bind(lon)
            .bind(config.search_radius)
            .fetch_optional(&mut *exec)
            .await?;

        if let Some(result) = result {
            result.get::<Uuid, _>("id")
        } else {
            sqlx::query(include_str!("sql/insert_one_station_ret_id.sql"))
                .bind(lat)
                .bind(lon)
                .fetch_one(&mut *exec)
                .await?
                .get::<Uuid, _>("id")
        }
    };

    // Now create the song history
    let _ = {
        let query = if let Some(world_id) = world_id {
            sqlx::query(include_str!("sql/insert_one_song_history_with_world.sql"))
                .bind(world_id)
        } else {
            sqlx::query(include_str!("sql/insert_one_song_history.sql"))
        };

        query.bind(user_id)
            .bind(station_id)
            .bind(song_id)
            .bind(lat)
            .bind(lon)
            .execute(&mut *exec)
            .await?;
    };

    Ok(())
}