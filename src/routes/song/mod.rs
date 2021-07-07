use actix_web::web;

pub(crate) mod handlers;
pub(crate) mod model;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(handlers::add_local_song);
    config.service(handlers::add_existing_song);
    config.service(handlers::get_songs_nearby);
}