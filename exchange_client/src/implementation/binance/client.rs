//! A client implementation for the Binance exchange.
//! The client is instantiated by [crate::api::provider].

use futures_util::Sink;
use crate::client_re_exports::*;

pub struct Binance {
    pub config: ExchangeClientConfig,
    sender: UnboundedSender<OrderbookSnapshot>,
    exchange: Exchange,
}

impl ExchangeClient for Binance {

    fn new(config: ExchangeClientConfig, sender: UnboundedSender<OrderbookSnapshot>) -> Self {
        Binance {
            config,
            sender,
            exchange: Exchange::Binance,
        }
    }

    fn build_url(&self) -> Url {
        let url_str: String = format!(
            "{}/{}@depth{}@100ms",
            self.config.base_url,
            self.config.symbol.to_string(),
            self.config.depth.to_string()
        );

        Url::parse(&url_str).unwrap()
    }

    async fn connect(&self, url: &Url) -> (impl Sink<Message>, impl Stream<Item=Result<Message, Error>> + Unpin) {
        println!("Connecting to `{}` : `{}`", self.exchange, url.as_str());

        let ws_stream = match connect_async(url.as_str()).await  {
            Ok((ws_stream, _)) => {
                println!("Connected to `{}` : `{}`", self.exchange, url.as_str());
                ws_stream
            }
            Err(error) => {
                panic!("Error connecting to `{}` : `{}`", self.exchange, error);
            }
        };
        ws_stream.split()
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
            |value| -> &serde_json::Value { &value["bids"] },
            |value| -> &serde_json::Value { &value["asks"] },
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
