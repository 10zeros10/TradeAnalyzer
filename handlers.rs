use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use futures::StreamExt;

#[derive(Deserialize, Serialize)]
struct TradeData {
    trade_id: u32,
    trade_amount: f64,
    trade_time: String,
}

#[derive(Serialize)]
struct AnalysisResult {
    average_trade_amount: f64,
    total_trade_count: usize,
}

async fn upload_trade_data(mut payload: web::Payload, data_path: String) -> impl Responder {
    match File::create(&data_path) {
        Ok(mut trade_data_file) => {
            while let Some(chunk) = payload.next().await {
                match chunk {
                    Ok(data) => {
                        if let Err(e) = trade_data_file.write_all(&data) {
                            return HttpResponse::InternalServerError().body(format!("Failed to write data: {:?}", e));
                        }
                    },
                    Err(e) => return HttpResponse::InternalServerError().body(format!("Error extracting chunk: {:?}", e)),
                }
            }

            HttpResponse::Ok().body("Trade data uploaded successfully")
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create file: {:?}", e)),
    }
}

async fn process_trade_data(data_path: String) -> impl Responder {
    let file = match File::open(&data_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to open file: {:?}", e)),
    };
    let reader = BufReader::new(file);

    let trades: Vec<TradeData> = match serde_json::from_reader(reader) {
        Ok(trades) => trades,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error while reading: {:?}", e)),
    };

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
    let results = AnalysisResult {
        average_trade_amount: 100.0,
        total_trade_count: 2,
    };

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let data_path = env::var("TRADE_DATA_PATH").unwrap_or_else(|_| "./trade_data.json".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data_path.clone()))
            .route("/upload", web::post().to(upload_trade_data))
            .route("/process", web::get().to(process_trade_data))
            .route("/results", web::get().to(get_analyzed_results))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}