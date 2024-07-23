
use crate::exchange_level::ExchangeLevel;

/// Represents the aggregated orderbook.
pub struct AggregatedOrderbook {
    pub bids: Vec<ExchangeLevel>,
    pub asks: Vec<ExchangeLevel>
}

impl AggregatedOrderbook {
    /// Constructs a new [AggregatedOrderbook]. 
    /// Called in the initialization of the Orderbook Aggregator.
    pub fn new(bids: Vec::<ExchangeLevel>, asks: Vec::<ExchangeLevel>) -> Self {
        AggregatedOrderbook {
            bids,
            asks
        }
    }

    pub fn spread(&self) -> f64 {
        return 0.0
    }
}
