use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct TradeData {
    trade_id: String,
    trade_amount: f32,
    trade_type: String,
}

async fn upload_file(bytes: web::Bytes) -> impl Responder {
    let size = bytes.len();
    HttpResponse::Ok().body(format!("File uploaded with {} bytes", size))
}

async fn process_trade(item: web::Json<TradeData>) -> impl Responder {
    HttpResponse::Ok().json(item.0)
}

async fn analyze_results() -> impl Responder {
    HttpResponse::Ok().body("Analysis results")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_file))
            .route("/trade", web::post().to(process_trade))
            .route("/analyze", web::get().to(analyze_results))
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}