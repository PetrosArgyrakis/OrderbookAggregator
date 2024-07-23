use crate::{exchange_level::Exchange, levels::Levels};

/// Contains a given exchange's orderbook snapshot.
/// It is published by the exchange client(s) and 
/// is consumed by the orderbook aggregator.
pub struct OrderbookSnapshot {
    pub exchange: Exchange,
    pub symbol: String,
    pub levels: Levels,
}

impl OrderbookSnapshot {
    /// Constructs a new [OrderbookSnapshot].
    ///
    /// # Arguments
    ///
    /// * `exchange` - The [Exchange] that published the orderbook update.
    /// * `symbol` - A [`Vec<Level>`] containing the asks of the orderbook.
    /// * `levels` - A [`Vec<Level>`] containing the asks of the orderbook.
    pub fn new(exchange: Exchange, symbol: String, levels: Levels) -> OrderbookSnapshot {
        OrderbookSnapshot {
            exchange,
            symbol,
            levels,
        }
    }
}
