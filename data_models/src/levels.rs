use std::fmt;

pub use crate::level::Level;

/// Represents the bids and asks of an orderbook
#[derive(Clone, PartialEq)]
pub struct Levels {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Levels {
    /// Constructs a new [Levels].
    ///
    /// # Arguments
    ///
    /// * `bids` - A [`Vec<Level>`] containing the bids of the orderbook.
    /// * `asks` - A [`Vec<Level>`] containing the asks of the orderbook.
    pub fn new(bids: Vec<Level>, asks: Vec<Level>) -> Self {
        Levels {
            bids: bids,
            asks: asks,
        }
    }
}

impl fmt::Debug for Levels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Levels")
            .field("bids", &self.bids)
            .field("asks", &self.asks)
            .finish()
    }
}
