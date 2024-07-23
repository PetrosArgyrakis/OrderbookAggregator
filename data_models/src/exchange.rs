use std::{fmt, str::FromStr};

/// Enumeration of the supported exchanges. It is required by any
/// exchange client implementation. To add support for new exchanges
/// add them in this enumeration and implement the match arms in
/// [`FromStr`] and [`fmt::Display`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exchange {
    Binance,
    Bitstamp,
}

impl FromStr for Exchange {
    type Err = (); // Define a custom error

    fn from_str(input: &str) -> Result<Exchange, Self::Err> {
        match input {
            "Binance" => Ok(Exchange::Binance),
            "Bitstamp" => Ok(Exchange::Bitstamp),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exchange::Binance => write!(f, "Binance"),
            Exchange::Bitstamp => write!(f, "Bitstamp"),
        }
    }
}
