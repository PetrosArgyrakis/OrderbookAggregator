use std::fmt;
pub use std::str::FromStr;

/// Enumeration of the available symbols.
/// To add support for new symbols add them in this enumeration
/// and implement the match arms in [`FromStr`] and [`fmt::Display`].
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Symbol {
    ETHBTC,
    BTCUSDT,
}

impl FromStr for Symbol {
    type Err = (); // Define a custom error

    fn from_str(input: &str) -> Result<Symbol, Self::Err> {
        match input {
            "ethbtc" => Ok(Symbol::ETHBTC),
            "btcusdt" => Ok(Symbol::BTCUSDT),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::ETHBTC => write!(f, "ethbtc"),
            Symbol::BTCUSDT => write!(f, "btcusdt"),
        }
    }
}
