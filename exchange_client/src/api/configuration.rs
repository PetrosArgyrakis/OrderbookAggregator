//! Configuration for the exchange clients implementing [crate::exchange_client::ExchangeClient].
//! The configuration for each client must be instantiated by the 
//! caller and then provided to [crate::api::provider].

use url::Url;

/// The exchange client configuration that is supplied to the [crate::api::provider].
pub struct ExchangeClientConfig {
    pub base_url: Url,
    pub depth: usize,
    pub symbol: String,
}

impl ExchangeClientConfig {
    /// Constructs a new [ExchangeClientConfig].
    ///
    /// # Arguments
    ///
    /// * `base_url` - A string that holds the base Url of the exchange.
    /// * `depth` - The maximum orderbook depth that will be streamed by the exchange client.
    /// * `symbol` - The [Symbol] that the exchange client will subscribe.
    ///
    /// This method will panic if:
    /// (1) the provided base_url is invalid and cannot be parsed
    /// (2) the provided symbol has not been implemented in [Symbol].
    pub fn new(base_url: String, depth: usize, symbol: String) -> Self {
        ExchangeClientConfig {
            base_url: Url::parse(&base_url).unwrap(),
            depth,
            symbol
        }
    }
}
