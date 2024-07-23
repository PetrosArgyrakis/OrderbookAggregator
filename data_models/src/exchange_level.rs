use std::fmt;

pub use crate::exchange::Exchange;
pub use crate::level::Level;

/// Contains a single [Level] of the orderbook of a given [Exchange].
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct ExchangeLevel {
    pub exchange: Exchange,
    pub level: Level,
}

impl ExchangeLevel {
    /// Constructs a new [ExchangeLevel].
    ///
    /// # Arguments
    ///
    /// * `exchange` - The given [Exchange].
    /// * `level` - The given [Level].
    pub fn new(exchange: Exchange, level: Level) -> ExchangeLevel {
        ExchangeLevel { exchange, level }
    }
}

impl fmt::Display for ExchangeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(E={}, L={})", self.exchange, self.level)
    }
}

impl fmt::Debug for ExchangeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExchangeLevel")
            .field("exchange", &self.exchange)
            .field("level", &self.level).finish()
    }
}