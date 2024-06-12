use std::env;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum TradeAnalyzerError {
    EnvVarError(env::VarError),
}

impl fmt::Display for TradeAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TradeAnalyzerError::EnvVarError(err) => write!(f, "Environment Variable Error: {}", err),
        }
    }
}

impl Error for TradeAnalyzerError {}

impl From<env::VarError> for TradeAnalyzerError {
    fn from(err: env::VarError) -> TradeAnalyzerError {
        TradeAnalyzerError::EnvVarError(err)
    }
}

fn load_env_var(key: &str) -> Result<String, TradeAnalyzerError> {
    env::var(key).map_err(TradeAnalyzerError::from)
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
    let database_url = match load_env_var("YOUR_ENV_VAR") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error loading the YOUR_ENV_VAR environment variable: {}", e);
            std::process::exit(1);
        }
    };
    println!("Database URL from env: {}", database_url);
}