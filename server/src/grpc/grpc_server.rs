use std::pin::Pin;

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc::UnboundedReceiver;
use tonic::{Request, Response, Status};
use tonic::async_trait;
use tonic::codegen::tokio_stream::Stream;

/// GRPC server implementation
use data_models::exchange_level::ExchangeLevel;
use data_models::exchange_orderbook::OrderbookSnapshot;
use orderbook::api::provider::AggregatorType;
use orderbook::api::provider::OrderbookSnapshotAggregator;
use grpc_orderbook::{Empty, Level, Summary};
use grpc_orderbook::orderbook_aggregator_server::OrderbookAggregator as GrpcOrderbookAggregator;

pub mod grpc_orderbook {
    tonic::include_proto!("orderbook");
}

pub struct Grpc {
    sender: broadcast::Sender<Result<Summary, Status>>,
}

impl Grpc {
    /// Constructs a new [Grpc].
    ///
    /// # Arguments
    ///
    /// * `orderbook_rx` - The [`UnboundedReceiver<OrderbookSnapshot>`] that receives the orderbook updates from
    /// exchange clients.
    ///
    /// Calling this method will spawn a [`tokio::task`] that will publish the aggregated orderbook to the connected clients.
    pub fn new(mut orderbook_rx: UnboundedReceiver<OrderbookSnapshot>) -> Grpc {
        let (tx, _) = broadcast::channel::<Result<Summary, Status>>(1000);
        let sender = tx.clone();

        tokio::spawn(async move {
            let mut aggregator = orderbook::api::provider::get(AggregatorType::HashMapOrderbookAggegator);

            while let Some(orderbook_snapshot) = orderbook_rx.recv().await {
                let aggregated_orderbook = aggregator.on_orderbook_snapshot(orderbook_snapshot);

                let summary = Summary {
                    spread: aggregated_orderbook.spread(),
                    bids: Self::transform(&aggregated_orderbook.bids),
                    asks: Self::transform(&aggregated_orderbook.asks),
                };

                match sender.send(Ok(summary)) {
                    Ok(_) => {}
                    // A send error will only occur if there are no active receivers
                    // https://docs.rs/tokio/latest/tokio/sync/broadcast/error/struct.SendError.html
                    Err(_) => {}
                }
            }
        });

        Grpc { sender: tx.clone() }
    }

    /// Transform function that converts the given asks or bids into the streaming gRPC data model
    ///
    /// # Arguments
    ///
    /// * `exchange_levels` - The [VecDeque<ExchangeLevel>] containing the bids or asks
    ///
    /// Returns the exchange_levels mapped into [Vec<Level>]
    #[inline]
    fn transform(exchange_levels: &Vec<ExchangeLevel>) -> Vec<Level> {
        exchange_levels
            .into_iter()
            .map(|exchange_level| Level {
                exchange: exchange_level.exchange.to_string(),
                price: exchange_level.level.price,
                amount: exchange_level.level.amount,
            })
            .collect()
    }

    fn get_receiver(&self) -> broadcast::Receiver<Result<Summary, Status>> {
        self.sender.subscribe()
    }
}


#[async_trait]
impl GrpcOrderbookAggregator for Grpc {
    type BookSummaryStream = Pin<Box<dyn Stream<Item=Result<Summary, Status>> + Send>>;

    async fn book_summary(&self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let mut receiver: broadcast::Receiver<Result<Summary, Status>> = self.get_receiver();

        let output = async_stream::stream! {
            loop {
                let result: Result<Result<Summary, Status>, RecvError> = receiver.recv().await;
                match result {
                    Ok(message) => yield message,
                    //TODO: handle the two possible errors
                    //https://docs.rs/tokio/latest/tokio/sync/broadcast/error/enum.RecvError.html
                    Err(err) => println!("{}", err)
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::BookSummaryStream))
    }
}
