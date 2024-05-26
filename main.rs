use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::error::PayloadError;
use actix_web::web::PayloadConfig;

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

fn error_response(status_code: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status_code).json(ErrorResponse { error: message.to_string() })
}

async fn upload_file(bytes: web::Bytes) -> impl Responder {
    let size = bytes.len();
    if size == 0 {
        return error_response(StatusCode::BAD_REQUEST, "File empty or not provided.");
    }
    HttpResponse::Ok().body(format!("File uploaded with {} bytes", size))
}

async fn process_trade(item: web::Json<TradeData>) -> impl Responder {
    if item.trade_amount <= 0.0 {
        return error_response(StatusCode::BAD_REQUEST, "Trade amount must be greater than 0.");
    }
    // Feel free to add more validations as needed
    HttpResponse::Ok().json(item.0)
}

async fn analyze_results() -> impl Responder {
    // Assuming this async block is replaced with actual logic that can fail
    let analysis_outcome = std::result::Result::<String, std::io::Error>::Err(std::io::Error::new(std::io::ErrorKind::Other, "Analysis Tool Failure"));

    match analysis_outcome {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Analysis failed: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Invalid PORT environment variable; using default port 8080.");
        8080
    });

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(PayloadConfig::new(1 << 25)) // Increase max payload size if needed (here set to 32MB)
            .route("/upload", web::post().to(upload_file))
            .route("/trade", web::post().to(process_trade))
            .route("/analyze", web::get().to(analyze_results))
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}