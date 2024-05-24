use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct TradeData {
    trade_id: String,
    trade_amount: f32,
    trade_type: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn error_response(message: &str) -> HttpResponse {
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(ErrorResponse { error: message.to_string() })
}

async fn upload_file(bytes: web::Bytes) -> impl Responder {
    let size = bytes.len();
    HttpResponse::Ok().body(format!("File uploaded with {} bytes", size))
}

async fn process_trade(item: web::Json<TradeData>) -> impl Responder {
    HttpResponse::Ok().json(item.0)
}

async fn analyze_results() -> impl Responder {
    match todo!() {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => error_response(&format!("Analysis failed: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Invalid PORT environment variable; using default port 8080.");
        8080
    });

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_file))
            .route("/trade", web::post().to(process_trade))
            .route("/analyze", web::get().to(analyze_results))
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}