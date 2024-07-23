//! A provider that starts a new exchange client.
//! This is the main entry point for any caller that wants to start a new client.

use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};

use data_models::{exchange::Exchange, exchange_orderbook::OrderbookSnapshot};

use crate::exchange_client::ExchangeClient;

use super::configuration::ExchangeClientConfig;

/// Calling this method will:
///
/// (1) Instantiate a new [ExchangeClient]
///
/// (2) Start the new client
///
/// # Arguments
///
/// * `exchange` - The given [Exchange].
/// * `client_config` - The [ExchangeClientConfig] of the client.
/// * `sender` - The [`UnboundedSender<OrderbookSnapshot>`] channel where the client will publish
/// orderbook updates.
///
/// The method will panic if a client implementation for the provided exchange does not exist.
pub fn start(
    exchange: Exchange,
    client_config: ExchangeClientConfig,
    sender: UnboundedSender<OrderbookSnapshot>,
) -> JoinHandle<()> {
    match exchange {
        Exchange::Binance => {
            let client = crate::implementation::binance::client::Binance::new(client_config, sender);
            tokio::spawn(async move { client.start().await })
        }
        Exchange::Bitstamp => {
            let client = crate::implementation::bitstamp::client::Bitstamp::new(client_config, sender);
            tokio::spawn(async move { client.start().await })
        }
    }
}
