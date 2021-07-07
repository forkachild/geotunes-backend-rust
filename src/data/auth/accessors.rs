use sqlx::{PgPool, Row};
use sqlx::types::Uuid;

use crate::config::AppConfig;
use crate::errors::ManagedError;
use crate::types::ManagedResult;
use crate::utils::jwt;

pub async fn create_session(
    pool: &PgPool,
    config: &AppConfig,
    email: String,
    provider: String,
    token: String,
) -> ManagedResult<String> {
    let mut trx = pool.begin().await?;

    let user_id = {
        let result = sqlx::query(include_str!("sql/get_one_by_email.sql"))
            .bind(&email)
            .fetch_optional(&mut trx)
            .await?;

        if let Some(result) = result {
            result.get::<Uuid, _>("user_id")
        } else {
            let user_id = sqlx::query(include_str!("sql/insert_one_user_ret_id.sql"))
                .fetch_one(&mut trx)
                .await?
                .get::<Uuid, _>("id");

            let _ = sqlx::query(include_str!("sql/insert_one_user_details.sql"))
                .bind(&user_id)
                .bind(&email)
                .execute(&mut trx)
                .await?;

            user_id
        }
    };

    let result = sqlx::query(include_str!("sql/get_token_by_user_id_provider.sql"))
        .bind(&user_id)
        .bind(&provider)
        .fetch_optional(&mut trx)
        .await?;

    if let Some(result) = result {
        let existing_token = result.get::<String, _>("token");

        if token != existing_token {
            return Err(ManagedError::AlreadyExists);
        }
    } else {
        let _ = sqlx::query(include_str!("sql/insert_one_user_auth_provider.sql"))
            .bind(&user_id)
            .bind(&provider)
            .bind(&token)
            .execute(&mut trx)
            .await?;
    }

    let _ = trx.commit().await?;

    Ok(jwt::encode(config, &user_id)?)
}