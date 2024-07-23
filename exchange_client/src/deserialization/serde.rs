//! A [serde] visitor that deserializes an array of bids or asks.
//! The visitor is called indirectly by [super::levels::deserialize] during the 
//! deserialization of the exchange orderbook updates.

use std::fmt;

use serde::de::{self, Unexpected, Visitor};

use data_models::levels::Level;

pub struct OrderbookPriceSizeVisitor;

impl<'de> Visitor<'de> for OrderbookPriceSizeVisitor {
    type Value = Level;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a vector with 2 elements [\"<price>\", \"<amount>\"]")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let price = match seq.next_element::<String>()? {
            Some(val) => val
                .parse::<f64>()
                .map_err(|_| de::Error::invalid_type(Unexpected::Str(&val), &self)),
            None => Err(de::Error::missing_field("price")),
        }?;

        let amount = match seq.next_element::<String>()? {
            Some(val) => val
                .parse::<f64>()
                .map_err(|_| de::Error::invalid_type(Unexpected::Str(&val), &self)),
            None => Err(de::Error::missing_field("amount")),
        }?;

        Ok(Level { price, amount })
    }
}
