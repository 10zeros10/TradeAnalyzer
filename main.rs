use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Trade {
    id: String,
    amount: f32,
    trade_type: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn create_http_error_response(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse { error: message.to_string() })
}

async fn upload_file_handler(payload: web::Bytes) -> impl Responder {
    let file_size = payload.len();
    if file_size == 0 {
        return create_http_error_response(StatusCode::BAD_REQUEST, "File is empty or not provided.");
    }
    HttpResponse::Ok().body(format!("File uploaded with {} bytes", file_size))
}

async fn process_trade_request(trade_info: web::Json<Trade>) -> impl Responder {
    if trade_info.amount <= 0.0 {
        return create_http_error_response(StatusCode::BAD_REQUEST, "Trade amount must be positive.");
    }
    HttpResponse::Ok().json(trade_info.0) // Successfully returns trade info if it's valid
}

async fn execute_trade_analysis() -> impl Responder {
    let analysis_result = std::result::Result::<String, std::io::Error>::Err(std::io::Error::new(std::io::ErrorKind::Other, "Failure in analysis tool"));

    match analysis_result {
        Ok(analysis_data) => HttpResponse::Ok().json(analysis_data),
        Err(error) => create_http_error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Analysis failed: {}", error)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Invalid or missing PORT environment variable; defaulting to port 8080.");
        8080
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::PayloadConfig::new(1 << 25)) // 32 MB max payload size
            .route("/upload", web::post().to(upload_file_handler))
            .route("/trade", web::post().to(process_trade_request))
            .route("/analyze", web::get().to(execute_trade_analysis))
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}