use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct TradeData {
    trade_id: u32,
    trade_amount: f64,
    trade_time: String, // Consider using more efficient time representations
}

#[derive(Serialize)]
struct AnalysisResult {
    average_trade_amount: f64,
    total_trade_count: usize,
}

async fn upload_trade_data(mut payload: web::Payload, data_path: String) -> impl Responder {
    // Directly use the passed data_path
    let mut trade_data_file = File::create(&data_path).expect("Failed to create file");

    // Assuming payload processing to write to file is omitted for brevity
    while let Some(chunk) = payload.next().await {
        let data = chunk.expect("Error extracting chunk");
        trade_data_file.write_all(&data).expect("Failed to write data");
    }

    HttpResponse::Ok().body("Trade data uploaded successfully")
}

async fn process_trade_data(data_path: String) -> impl Responder {
    // Directly use the passed data_path.
    let file = File::open(&data_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let trades: Vec<TradeData> = serde_json::from_reader(reader).expect("Error while reading");

    let total_trade_amount: f64 = trades.iter().map(|trade| trade.trade_amount).sum();
    let total_trade_count = trades.len();
    let average_trade_amount = if total_trade_count > 0 {
        total_trade_amount / total_trade_count as f64
    } else {
        0.0
    };

    let analysis_result = AnalysisResult {
        average_trade_amount,
        total_trade_count,
    };

    HttpResponse::Ok().json(analysis_result)
}

async fn get_analyzed_results(data_path: String) -> impl Responder {
    // This example function does not utilize the data path for dynamic results. Consider integrating real data.
    let results = AnalysisResult {
        average_trade_amount: 100.0,
        total_trade_count: 2,
    };

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Fetch the path once and reuse it.
    let data_path = env::var("TRADE_DATA_PATH").unwrap_or("./trade_data.json".to_string());

    HttpServer::new(move || {
        App::new()
            .data(data_path.clone()) // Pass data_path as app data
            .route("/upload", web::post().to(upload_trade_data))
            .route("/process", web::get().to(process_trade_data))
            .route("/results", web::get().to(get_analyzed_results))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}