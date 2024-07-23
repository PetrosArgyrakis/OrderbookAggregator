use clap::{arg, Parser};

use data_models::{exchange::Exchange, exchange_orderbook::OrderbookSnapshot};
use exchange_client::api::configuration::ExchangeClientConfig;

mod grpc;

/// The command line arguments the server can parse.
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    symbol: String,

    #[arg(short, long, default_value_t = String::from("[::1]:50051"))]
    address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (tx_exchange, rx_exchange) = tokio::sync::mpsc::unbounded_channel::<OrderbookSnapshot>();
    let depth = 10;

    exchange_client::api::provider::start(
        Exchange::Bitstamp,
        ExchangeClientConfig::new(
            String::from("wss://ws.bitstamp.net"),
            depth,
            args.symbol.clone(),
        ),
        tx_exchange.clone(),
    );

    exchange_client::api::provider::start(
        Exchange::Binance,
        ExchangeClientConfig::new(
            String::from("wss://stream.binance.com:9443/ws"),
            depth,
            args.symbol.clone(),
        ),
        tx_exchange.clone(),
    );

    let server = grpc::provider::start(rx_exchange, &args.address);

    match server.await {
        Ok(_) => println!("Server stopped"),
        Err(e) => println!("Server terminated with error {}", e.to_string()),
    }
}
