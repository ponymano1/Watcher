use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub chain_id: i64,
    pub scan_interval: Duration,
    pub confirmations: u64,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let server_address = std::env::var("SERVER_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let chain_id = std::env::var("CHAIN_ID")
            .ok()
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or(1);
        let scan_interval_secs = std::env::var("SCAN_INTERVAL")
            .ok()
            .and_then(|interval| interval.parse::<u64>().ok())
            .unwrap_or(5);
        let confirmations = std::env::var("CONFIRMATIONS")
            .ok()
            .and_then(|confirmations| confirmations.parse::<u64>().ok())
            .unwrap_or(0);
        
        Self { 
            database_url, 
            server_address, 
            chain_id, 
            scan_interval: Duration::from_secs(scan_interval_secs), 
            confirmations }
    }
}