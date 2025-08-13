use actix_web::{get, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct Params {
    pub coin: String,
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {}", name)
}



#[get("/fetch")]
async fn fetch_price(query: web::Query<Params>) -> impl Responder {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", query.coin);
    match reqwest::get(&url).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => if text == "{}" {
                    HttpResponse::NotFound().json(ErrorResponse {
                        error: format!("Coin '{}' not found", query.coin),
                    })
                } else {
                    HttpResponse::Ok().content_type("application/json").body(text)
                }   
                Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to parse response".to_string(),
                }),
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch data".to_string(),
        }),
    }
}

#[get("/coins")]
async fn list() -> impl Responder {
    let url = "https://api.coingecko.com/api/v3/coins/list";
    match reqwest::get(url).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => HttpResponse::Ok().content_type("application/json").body(text),
                Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to parse response".to_string(),
                }),
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch data".to_string(),
        }),
    }
}