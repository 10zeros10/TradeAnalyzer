use std::env;
use serde::{Deserialize, Serialize};

fn load_env_var(key: &str) -> String {
    env::var(key).expect(&format!("Error loading the {} environment variable.", key))
}

#[derive(Debug, Serialize, Deserialize)]
struct Trade {
    trade_id: u32,
    trader_name: String,
    symbol: String,
    size: u32,
    price: f64,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResult {
    average_price: f64,
    max_trade_size: u32,
    total_volume: u32,
    timestamp: u64,
}

fn main() {
    let database_url = load_env_var("YOUR_ENV_VAR");
    println!("Database URL from env: {}", database_url);
}