use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

<<<<<<< HEAD
pub mod utils;
pub mod routes;
=======
mod utils;
mod routes;
>>>>>>> 815da385e2c8327b9a34c7ab304ecc33e84ee1e0



#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let address: String = utils::constants::ADDRESS.clone();
    let port: u16 = utils::constants::PORT.clone();
    dotenv::dotenv().ok();
    println!("Server running at http://{}:{}", address, port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
