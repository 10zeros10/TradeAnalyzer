use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use futures::StreamExt;

#[derive(Deserialize, Serialize)]
struct Trade {
    id: u32,
    amount: f64,
    time: String,
}

#[derive(Serialize)]
struct TradeAnalysis {
    average_amount: f64,
    total_count: usize,
}

async fn upload_trades(mut payload: web::Payload, data_file_path: String) -> impl Responder {
    match File::create(&data_file_path) {
        Ok(mut file) => {
            while let Some(chunk) = payload.next().await {
                match chunk {
                    Ok(data) => {
                        if let Err(e) = file.write_all(&data) {
                            return HttpResponse::InternalServerError().body(format!("Failed to write data: {:?}", e));
                        }
                    },
                    Err(e) => return HttpResponse::InternalServerError().body(format!("Error extracting chunk: {:?}", e)),
                }
            }

            HttpResponse::Ok().body("Trades uploaded successfully")
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create file: {:?}", e)),
    }
}

async fn analyze_trade_data(file_path: String) -> impl Responder {
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to open file: {:?}", e)),
    };
    let reader = BufReader::new(file);

    let trades: Vec<Trade> = match serde_json::from_reader(reader) {
        Ok(trades) => trades,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error while reading trades: {:?}", e)),
    };

    let total_amount: f64 = trades.iter().map(|trade| trade.amount).sum();
    let trade_count = trades.len();
    let average_amount = if trade_count > 0 {
        total_amount / trade_count as f64
    } else {
        0.0
    };

    let analysis_result = TradeAnalysis {
        average_amount,
        total_count: trade_count,
    };

    HttpResponse::Ok().json(analysis_result)
}

async fn fetch_analysis_results(_file_path: String) -> impl Responder {
    let dummy_results = TradeAnalysis {
        average_amount: 100.0,
        total_count: 2,
    };

    HttpResponse::Ok().json(dummy_results) // Note: This function now fetches dummy results. Integrate with actual analysis if needed.
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let trade_data_file_path = env::var("TRADE_DATA_PATH").unwrap_or_else(|_| "./trade_data.json".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(trade_data_file_path.clone()))
            .route("/upload", web::post().to(upload_trades))
            .route("/analyze", web::get().to(analyze_trade_data)) // Changed from /process to /analyze for clarity.
            .route("/results", web::get().to(fetch_analysis_results))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}