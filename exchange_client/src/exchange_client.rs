//! A common trait implemented by the exchange clients.
//! Example implemetations can be found here: [crate::binance::client::Binance]
//! and here: [crate::bitstamp::client::Bitstamp].

use futures_util::{Sink, Stream, StreamExt};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use url::Url;

use data_models::exchange_orderbook::OrderbookSnapshot;

use crate::api::configuration::ExchangeClientConfig;
use crate::client_re_exports::{Error, Message};

pub trait ExchangeClient: Sized {
    /// Constructs a new exchange client.
    ///
    /// # Arguments
    ///
    /// * `client_config` - The [ExchangeClientConfig] for the client.
    /// * `sender` - The [`UnboundedSender<OrderbookSnapshot>`] channel where the client will publish orderbook updates.
    fn new(config: ExchangeClientConfig, sender: UnboundedSender<OrderbookSnapshot>) -> Self;

    /// This is the entry point for an exchange client implementation.
    async fn start(&self) {
        let url = self.build_url();
        loop {
            let (sink, stream) = self.connect(&url).await;
            self.process_stream(sink, stream).await
        }
    }

    /// The implementation creates the connection
    /// url using the given [ExchangeClientConfig].
    fn build_url(&self) -> Url;

    /// The implementation will establish a connection to the given exchange
    /// and return a stream that can be processed by [ExchangeClient::process_stream].
    async fn connect(&self, url: &Url) -> (impl Sink<Message>, impl Stream<Item=Result<Message, Error>> + Unpin);

    /// The implementation will call this method after the client has connected to
    /// the exchange and is ready to receive orderbook updates.
    async fn process_stream(&self, sink: impl Sink<Message>, mut steam: impl Stream<Item=Result<Message, Error>> + Unpin) {
        while let Some(message) = steam.next().await {
            _ = match message {
                Ok(message) => {
                    if message.is_text() {
                        match message.into_text() {
                            Ok(message_str) => self.deserialize(&message_str),
                            Err(err) => panic!("Unexpected error {}", err)
                        }
                        continue;
                    }

                    if message.is_ping() {
                        self.on_ping(&message);
                        continue;
                    }

                    if message.is_pong() {
                        self.on_pong(&message);
                        continue;
                    }

                    if message.is_close() {
                        self.on_close(&message);
                        break;
                    }

                    if message.is_empty() {
                        continue;
                    }

                    if message.is_binary() {
                        panic!("Unexpected binary message {}", message);
                    }
                }
                Err(err) => panic!("Unexpected error {}", err)
            }
        }
    }

    fn on_ping(&self, message: &Message);

    fn on_pong(&self, message: &Message);

    fn on_close(&self, message: &Message);

    /// Deserializes an exchange message.
    fn deserialize(&self, message: &String);

    /// Action to perform when the message with the orderbook snapshot
    /// has been deserialized successfully.
    fn on_deserialized(&self, sender: &UnboundedSender<OrderbookSnapshot>,
                       exchange_orderbook: OrderbookSnapshot) {
        match sender.send(exchange_orderbook) {
            Ok(_) => (),
            Err(err) => Self::on_send_error(err),
        }
    }

    /// Action to perform when the exchange
    /// message could not be deserialized.
    fn on_deserialization_error(&self, error: serde_json::Error) {
        println!("{}", error)
    }

    fn on_send_error(error: SendError<OrderbookSnapshot>) {
        println!("{}", error)
    }
}
//
// //#[rustfmt::skip]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::deserialization::levels;
//     use data_models::levels::Level;
//     use futures_util::stream;
//     use rstest::rstest;
//     use serde_json::json;
//
//     struct ExchangeClientFixture {
//         depth: usize,
//         sender: UnboundedSender<Levels>,
//         stream_messages: Vec<String>,
//     }
//
//     #[async_trait]
//     impl ExchangeClient for ExchangeClientFixture {
//         type T = Levels;
//
//         fn new(_: ExchangeClientConfig, _: UnboundedSender<Levels>) -> Self {
//             panic!("Not implemented")
//         }
//
//         fn build_url(&self) -> Url {
//             Url::parse("http://www.somebaseurl.com").unwrap()
//         }
//
//         async fn connect(&self, _: &Url) -> Box<dyn Stream<Item = String> + Send> {
//             Box::new(stream::iter(self.stream_messages.clone()))
//         }
//
//         fn deserialize(&self, message: &String) -> LevelDeserializationResult {
//             levels::deserialize(
//                 self.depth,
//                 message,
//                 |value| -> &serde_json::Value { &value["bids"] },
//                 |value| -> &serde_json::Value { &value["asks"] },
//             )
//         }
//
//         fn on_deserialized(&self, levels: Levels) {
//             self.sender.send(Levels::new(levels.bids, levels.asks));
//         }
//
//         fn on_deserialization_error(&self, message: &String, error: serde_json::Error) {
//             // Currently this method does cause any side-effects in the exchange client implementations.
//             // This side effect (sending empty Levels) is for testing only.
//             self.sender
//                 .send(Levels::new(Vec::<Level>::new(), Vec::<Level>::new()));
//         }
//     }
//
//     #[rstest]
//     async fn deseriaze_messages() {
//         let stream_messages = vec![
//             json!( { "bids": [["1.0", "99"]], "asks": [["2.0", "101"]] } ).to_string(),
//             json!( { "bids": [["0.9", "98"]], "asks": [["2.1", "102"]] } ).to_string(),
//         ];
//
//         let (tx_exchange, mut rx_exchange) = tokio::sync::mpsc::unbounded_channel::<Levels>();
//
//         let exchange_client = ExchangeClientFixture {
//             depth: 1,
//             sender: tx_exchange.clone(),
//             stream_messages,
//         };
//
//         exchange_client.start().await;
//
//         let first_level_published = rx_exchange.blocking_recv();
//         assert_eq!(
//             first_level_published.as_ref().unwrap().bids,
//             vec![Level::new(1.0, 99.0)]
//         );
//         assert_eq!(
//             first_level_published.as_ref().unwrap().asks,
//             vec![Level::new(2.0, 101.0)]
//         );
//
//         let second_level_published = rx_exchange.blocking_recv();
//         assert_eq!(
//             second_level_published.as_ref().unwrap().bids,
//             vec![Level::new(0.9, 98.0)]
//         );
//         assert_eq!(
//             second_level_published.as_ref().unwrap().asks,
//             vec![Level::new(2.1, 102.0)]
//         );
//     }
//
//     #[rstest]
//     async fn deserialization_error() {
//         let stream_messages = vec![
//             json!( { "bids": [["1.0", "99"]], "asks": [["2.0", "101"]] } ).to_string(),
//             json!( { "bids": [["99"]], "asks": [["2.0", "101"]] } ).to_string(), //will cause deserialization error
//             json!( { "bids": [["0.9", "98"]], "asks": [["2.1", "102"]] } ).to_string(),
//         ];
//
//         let (tx_exchange, mut rx_exchange) = tokio::sync::mpsc::unbounded_channel::<Levels>();
//
//         let exchange_client = ExchangeClientFixture {
//             depth: 1,
//             sender: tx_exchange.clone(),
//             stream_messages,
//         };
//
//         exchange_client.start().await;
//
//         let first_level_published = rx_exchange.blocking_recv();
//         assert_eq!(
//             first_level_published.as_ref().unwrap().bids,
//             vec![Level::new(1.0, 99.0)]
//         );
//         assert_eq!(
//             first_level_published.as_ref().unwrap().asks,
//             vec![Level::new(2.0, 101.0)]
//         );
//
//         let second_level_published = rx_exchange.blocking_recv();
//         assert_eq!(second_level_published.as_ref().unwrap().bids, vec![]);
//         assert_eq!(second_level_published.as_ref().unwrap().asks, vec![]);
//
//         let third_level_published = rx_exchange.blocking_recv();
//         assert_eq!(
//             third_level_published.as_ref().unwrap().bids,
//             vec![Level::new(0.9, 98.0)]
//         );
//         assert_eq!(
//             third_level_published.as_ref().unwrap().asks,
//             vec![Level::new(2.1, 102.0)]
//         );
//     }
// }
