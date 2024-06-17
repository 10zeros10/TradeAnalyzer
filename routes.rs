use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;
use std::io::Write;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Deserialize)]
struct TradeData {
    data: Vec<u8>,
}

async fn upload_trade_data(mut item: web::Json<TradeData>) -> impl Responder {
    match save_trade_data(&mut item.data).await {
        Ok(_) => HttpResponse::Ok().json("Trade data uploaded and saved successfully."),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to save trade data: {}", e)),
    }
}

async fn process_trade_data() -> impl Responder {
    HttpResponse::Ok().json("Trade data processed successfully.")
}

async fn retrieve_analyzed_results() -> impl Responder {
    HttpResponse::Ok().json("Analyzed trade data results.")
}

async fn summarize_trade_data() -> impl Responder {
    match read_and_summarize_trade_data().await {
        Ok(summary) => HttpResponse::Ok().json(format!("Trade data summary: {}", summary)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to summarize trade data: {}", e)),
    }
}

async fn read_and_summarize_trade_data() -> std::io::Result<String> {
    let filepath = "trade_data.txt";
    let data = tokio::fs::read_to_string(filepath).await?;
    let num_entries = data.lines().count();
    Ok(format!("Total trade entries: {}", num_entries))
}

async fn save_trade_data(data: &Vec<u8>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("trade_data.txt")
        .await
        .expect("Failed to open trade_data.txt");

    file.write_all(data)
        .await
        .expect("Failed to write data to file");

    Ok(())
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
            .route("/summarize", web::get().to(summarize_trade_data))
    })
    .bind(bind_address)?
    .run()
    .await
}