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
struct ErrorMsg {
    error: String,
}

fn create_error_response(status: StatusCode, error_message: &str) -> HttpResponse {
    HttpResponse::build(status).json(ErrorMsg { error: error_message.to_string() })
}

async fn handle_file_upload(payload: web::Bytes) -> impl Responder {
    let payload_size = payload.len();
    if payload_size == 0 {
        return create_error_response(StatusCode::BAD_REQUEST, "File empty or not provided.");
    }
    HttpResponse::Ok().body(format!("File uploaded with {} bytes", payload_size))
}

async fn validate_and_process_trade(trade_data: web::Json<Trade>) -> impl Responder {
    if trade_data.amount <= 0.0 {
        return create_error_response(StatusCode::BAD_REQUEST, "Trade amount must be greater than 0.");
    }
    HttpResponse::Ok().json(trade_data.0)
}

async fn perform_analysis() -> impl Responder {
    let simulated_analysis_result = std::result::Result::<String, std::io::Error>::Err(std::io::Error::new(std::io::ErrorKind::Other, "Analysis Tool Failure"));

    match simulated_analysis_result {
        Ok(analysis) => HttpResponse::Ok().json(analysis),
        Err(err) => create_error_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Analysis failed: {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Invalid PORT environment variable; using default port 8080.");
        8080
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::PayloadConfig::new(1 << 25))
            .route("/upload", web::post().to(handle_file_upload))
            .route("/trade", web::post().to(validate_and_process_trade))
            .route("/analyze", web::get().to(perform_analysis))
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}