use actix_web::{get, post, web};
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::data::auth;
use crate::extractor::auth::Auth;
use crate::routes::auth::model::request;
use crate::types::{IntoResponseResult, ResponseResult};

#[post("/login-social")]
pub async fn login_social(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    data: web::Json<request::login::LoginUser>,
) -> ResponseResult<String> {
    let pool = pool.get_ref();
    let data = data.into_inner();
    let config = config.get_ref();

    let session = auth::create_session(
        pool,
        config,
        data.email,
        data.provider,
        data.token,
    ).await?;

    session.into_response()
}

#[get("/login-silent")]
pub async fn login_silent(
    _: Auth
) -> ResponseResult<()> {
    ().into_response()
}