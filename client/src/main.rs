/*!
A simple grpc client that prints the aggregated orderbook summary on every update
 */

use clap::Parser;
use tokio::{sync::broadcast, task};

use orderbook::{Empty, orderbook_aggregator_client, Summary};
use terminal_ui::start;

mod terminal_ui;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

/// The command line arguments the client can parse.
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("http://[::1]:50051"))]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut client =
        orderbook_aggregator_client::OrderbookAggregatorClient::connect(args.address)
            .await
            .unwrap();

    let mut stream = client.book_summary(Empty {}).await.unwrap().into_inner();

    let (tx_summary, rx_summary) = broadcast::channel::<Summary>(1000);

    let stream_handle = task::spawn(async move {
        while let Ok(Some(message)) = stream.message().await {
            match tx_summary.send(message) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    });

    let ui_handle = start(rx_summary);

    stream_handle.await;
    ui_handle.await;

    Ok(())
}
