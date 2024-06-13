use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

async fn upload_trade_data(item: web::Json<TradeData>) -> impl Responder {
    HttpResponse::Ok().json("Trade data uploaded successfully.")
}

async fn process_trade_data() -> impl Responder {
    HttpResponse::Ok().json("Trade data processed successfully.")
}

async fn retrieve_analyzed_results() -> impl Responder {
    HttpResponse::Ok().json("Analyzed trade data results.")
}

#[derive(serde::Deserialize)]
struct TradeData {
    data: Vec<u8>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let bind_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".into());

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_trade_data))
            .route("/process", web::get().to(process_trade_data))
            .route("/results", web::get().to(retrieve_analyzed_results))
    })
    .bind(bind_address)?
    .run()
    .await
}