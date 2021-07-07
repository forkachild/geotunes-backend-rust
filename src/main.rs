use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};

use crate::config::AppConfig;
use crate::errors::ManagedError;
use crate::types::ManagedResult;

pub(crate) mod data;
pub(crate) mod routes;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod extractor;
pub(crate) mod errors;
pub(crate) mod mapper;
pub(crate) mod config;

#[actix_web::main]
async fn main() -> ManagedResult<()> {
    #[cfg(debug_assertions)]
        dotenv::dotenv().ok();

    let config = web::Data::new(AppConfig::from_env()?);
    let port = config.port;

    println!("Setup database");
    let pool = data::db::create_pool(config.get_ref()).await?;
    let _ = data::db::create_schema(&pool).await?;

    let query_config = web::QueryConfig::default()
        .error_handler(|err, _| ManagedError::Query(err).into());

    let json_config = web::JsonConfig::default()
        .error_handler(|err, _| ManagedError::Json(err).into());

    let path_config = web::PathConfig::default()
        .error_handler(|err, _| ManagedError::Path(err).into());

    println!("Starting server");
    let _ = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(config.clone())
            .app_data(query_config.clone())
            .app_data(json_config.clone())
            .app_data(path_config.clone())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .configure(routes::configure)
            .default_service(web::to(routes::not_found))
    }).bind(format!("0.0.0.0:{}", port))?.run().await?;

    Ok(())
}
