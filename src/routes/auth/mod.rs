use actix_web::web;

mod handlers;
mod model;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(handlers::login_social);
    config.service(handlers::login_silent);
}