use actix_web::web;
use super::handlers;


pub fn config(config: &mut web::ServiceConfig) {
    config.service(handlers::home_handler::greet);
    config.service(handlers::home_handler::fetch_price);
    config.service(handlers::home_handler::list);
<<<<<<< HEAD
    config.service(handlers::home_handler::historical);
    config.service(handlers::home_handler::exchange);
    config.service(handlers::home_handler::find_stats);
    config.service(handlers::home_handler::register);
=======
>>>>>>> 815da385e2c8327b9a34c7ab304ecc33e84ee1e0
}