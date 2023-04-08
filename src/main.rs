use clap::Parser;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime};

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    p: f64,
    s: i64,
    t: String,
    c: Vec<String>,
    z: String,
    x: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AlpacaResponse {
    symbol: String,
    trade: Trade,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    symbol: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let url = format!(
        "https://data.alpaca.markets/v2/stocks/{}/trades/latest",
        args.symbol
    );

    let key = std::env::var("APCA_API_KEY_ID").expect("Error: APCA_API_KEY_ID not found");
    let secret =
        std::env::var("APCA_API_SECRET_KEY").expect("Error: APCA_API_SECRET_KEY not found");

    let client = Client::new();

    let response = client
        .get(url)
        .header("APCA-API-KEY-ID", key)
        .header("APCA-API-SECRET-KEY", secret)
        .send()
        .ok()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let alpaca_response = response.json::<AlpacaResponse>()?;
            let price = format!("${:.2}", alpaca_response.trade.p);
            
            let timestamp = DateTime::parse_from_rfc3339(&alpaca_response.trade.t)?;
            
            println!("{} {}: {}", timestamp.to_rfc2822(), args.symbol, price);
        }
        _ => {
            eprintln!("Error: {}", response.status());
        }
    }

    Ok(())
}
