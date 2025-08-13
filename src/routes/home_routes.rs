use actix_web::web;
use super::handlers;


pub fn config(config: &mut web::ServiceConfig) {
    config.service(handlers::home_handler::greet);
    config.service(handlers::home_handler::fetch_price);
    config.service(handlers::home_handler::list);
}