use actix_web::web;

mod handlers;
pub(crate) mod model;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(handlers::create_world);
    config.service(handlers::get_worlds);
}