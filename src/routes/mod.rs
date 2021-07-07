use actix_web::web;

use crate::errors::ManagedError;
use crate::types::ResponseResult;

pub(crate) mod song;
pub(crate) mod model;
pub(crate) mod auth;
pub(crate) mod world;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(web::scope("/auth").configure(auth::configure));
    config.service(web::scope("/song").configure(song::configure));
    config.service(web::scope("/world").configure(world::configure));
}

pub async fn not_found(req: web::HttpRequest) -> ResponseResult<()> {
    ResponseResult::<()>::Err(ManagedError::BadRoute(
        req.method().to_string(),
        req.uri().path().to_owned(),
    ))
}