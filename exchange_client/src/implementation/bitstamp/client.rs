//! A client implementation for the Bitstamp exchange.
//! The client is instantiated by [crate::api::provider].

use futures_util::Sink;

use data_models::exchange_orderbook::OrderbookSnapshot;

use crate::client_re_exports::*;

pub struct Bitstamp {
    config: ExchangeClientConfig,
    sender: UnboundedSender<OrderbookSnapshot>,
    exchange: Exchange,
}

impl ExchangeClient for Bitstamp {

    fn new(config: ExchangeClientConfig, sender: UnboundedSender<OrderbookSnapshot>) -> Self {
        Bitstamp {
            config,
            sender,
            exchange: Exchange::Bitstamp,
        }
    }

    fn build_url(&self) -> Url {
        self.config.base_url.clone()
    }

    async fn connect(&self, url: &Url) -> (impl Sink<Message>, impl Stream<Item=Result<Message, Error>> + Unpin) {
        println!("Connecting to `{}` : `{}`", self.exchange, url.as_str());

        let ws_stream = match connect_async(url.as_str()).await {
            Ok((ws_stream, _)) => {
                println!("Connected to `{}` : `{}`", self.exchange, url.as_str());
                ws_stream
            }
            Err(error) => {
                panic!("Error connecting to `{}` : `{}`", self.exchange, error);
            }
        };

        let (mut ws_write_stream, mut ws_read_stream) = ws_stream.split();
        self.subscribe(&mut ws_write_stream, &mut ws_read_stream).await;
        (ws_write_stream, ws_read_stream)
    }

    fn on_ping(&self, message: &Message) {
        println!("{}", message)
    }

    fn on_pong(&self, message: &Message) {
        println!("{}", message)
    }

    fn on_close(&self, message: &Message) {
        println!("{}", message)
    }

    fn deserialize(&self, message: &String) {
        _ = match levels::deserialize(
            self.config.depth,
            message,
            |value| -> &serde_json::Value { &value["data"]["bids"] },
            |value| -> &serde_json::Value { &value["data"]["asks"] },
        ) {
            Ok(levels) => {
                let snapshot = OrderbookSnapshot::new(
                    self.exchange,
                    self.config.symbol.clone(),
                    levels);
                self.on_deserialized(&self.sender, snapshot);
            }
            Err(error) => self.on_deserialization_error(error)
        }
    }
}

impl Bitstamp {
    async fn subscribe(
        &self,
        ws_write_stream: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        ws_read_stream: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let subscription_msg =
            super::subscription::Subscription::new(&self.config.symbol.to_string()).serialize();

        match ws_write_stream.send(Message::text(&subscription_msg)).await {
            Ok(()) => {
                println!(
                    "Subscribing to `{}` : `{}`",
                    self.exchange,
                    &subscription_msg
                )
            }
            Err(error) => {
                panic!(
                    "Could not subscribe to `{}` : `{}`",
                    self.exchange,
                    error
                )
            }
        }

        match ws_read_stream.next().await {
            None => {}
            Some(result) => match result {
                Ok(message) => {
                    println!("Subscribed successfully to `{}` : `{}`", self.exchange, message.into_text().unwrap())
                }
                Err(err) => {
                    panic!("Could not subscribe to `{}` : `{}`", self.exchange, err)
                }
            },
        }
    }
}
