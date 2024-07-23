//! Methods to deserialize exchange full orderbook snapshots into [Levels]
//! The methods would be called by [crate::exchange_client::ExchangeClient::deserialize]

use serde::{de::Expected, Deserializer};
use serde_json::Value;

use data_models::levels::{Level, Levels};

pub type LevelDeserializationResult = Result<Levels, serde_json::Error>;

/// Initialized by [deserialize_level] and passed as an argument to [serde::de::Error::invalid_length] error.
struct InvalidJsonArrayLength {
    expected_length: usize,
}

impl Expected for InvalidJsonArrayLength {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a json array with length {}",
            self.expected_length
        )
    }
}

/// Deserializes the bids and asks of exchange orderbook updates.
///
/// # Arguments
///
/// * `depth` - the number of levels to be deserialize.
/// * `json_data` - the bids and asks in json format.
/// * `bids` - a closure that accepts the root [Value] and returns a [Value] containing only the bids.
/// * `asks` - a closure that accepts the root [Value] and returns a [Value] containing only the asks.
pub fn deserialize(
    depth: usize,
    json_data: &String,
    bids: impl Fn(&Value) -> &Value + Sync,
    asks: impl Fn(&Value) -> &Value + Sync,
) -> LevelDeserializationResult {
    let json_value: Value = serde_json::from_str(json_data)?;

    let (bids, asks) = rayon::join(
        || deserialize_level(depth, bids(&json_value)),
        || deserialize_level(depth, asks(&json_value)),
    );

    match (bids, asks) {
        (Ok(bids), Ok(asks)) => Ok(Levels::new(bids, asks)),
        (Ok(_), Err(asks_err)) => Err(serde::de::Error::custom(format!(
            "Asks error: {}",
            asks_err.to_string()
        ))),
        (Err(bids_err), Ok(_)) => Err(serde::de::Error::custom(format!(
            "Bids error: {}",
            bids_err.to_string()
        ))),
        (Err(bids_err), Err(asks_err)) => Err(serde::de::Error::custom(format!(
            "Bids error: {}, Asks error: {}",
            bids_err.to_string(),
            asks_err.to_string()
        ))),
    }
}

/// Deserializes the exchange orderbook bids or asks and returns a vector of [Levels].
///
/// # Arguments
///
/// * `depth` - the number of levels to deserialize.
/// * `json_value` - the [Value] that stores the bids or asks.
fn deserialize_level(depth: usize, json_value: &Value) -> Result<Vec<Level>, serde_json::Error> {
    let mut levels = Vec::<Level>::with_capacity(depth);

    for index in 0..depth {
        let level = json_value
            .get(index)
            .ok_or_else(|| {
                serde::de::Error::invalid_length(
                    index,
                    &InvalidJsonArrayLength {
                        expected_length: depth,
                    },
                )
            })?
            .deserialize_seq(super::serde::OrderbookPriceSizeVisitor {})?;

        levels.push(level);
    }

    Ok(levels)
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use rstest::*;
    use serde_json::json;

    use super::*;

    #[rstest]
    #[case(1, json ! ([["1.0", "100"]]), vec ! [Level::new(1.0, 100.0)])]
    #[case(1, json ! ([["1.0", "100"], ["-1.0", "-100"]]), vec ! [Level::new(1.0, 100.0)])]
    #[case(
        2, json ! ([["1.0", "100"], ["-1.0", "-100"]]), vec ! [Level::new(1.0, 100.0), Level::new(- 1.0, - 100.0)]
    )]
    fn deserialize_level_succeeds(
        #[case] depth: usize,
        #[case] json_value: Value,
        #[case] expected: Vec<Level>,
    ) {
        let result = deserialize_level(depth, &json_value);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.as_ref().unwrap(), &expected);
        assert_eq!(result.as_ref().unwrap().capacity(), depth);
    }

    #[rstest]
    #[case(
        2, json ! ([["1.0", "100"]]), serde::de::Error::invalid_length(1, & InvalidJsonArrayLength { expected_length: 2 } )
    )]
    #[case(1, json ! ([[]]), serde::de::Error::missing_field("price"))]
    #[case(1, json ! ([["100"]]), serde::de::Error::missing_field("amount"))]
    #[case(
        1, json ! ([["invalid-float"]]), serde::de::Error::custom("invalid type: string \"invalid-float\", expected a vector with 2 elements [\"<price>\", \"<amount>\"]")
    )]
    fn deserialize_level_fails(
        #[case] depth: usize,
        #[case] json_value: Value,
        #[case] expected: serde_json::Error,
    ) {
        let result = deserialize_level(depth, &json_value);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap().to_string(), expected.to_string());
    }

    #[rstest]
    #[case(
        1, json ! ( { "bids": [["1.0", "99"]], "asks": [["2.0", "101"]] } ).to_string(), Levels::new(vec ! [Level::new(1.0, 99.0)], vec ! [Level::new(2.0, 101.0)])
    )]
    #[case(
        2, json ! ( { "bids": [["1.0", "99"], ["0.9", "98"]], "asks": [["2.0", "101"], ["2.1", "102"]] } ).to_string(),
        Levels::new(vec ! [Level::new(1.0, 99.0), Level::new(0.9, 98.0)], vec ! [Level::new(2.0, 101.0), Level::new(2.1, 102.0)])
    )]
    fn deserialize_succeeds(#[case] depth: usize, #[case] msg: String, #[case] expected: Levels) {
        let result = deserialize(
            depth,
            &msg,
            |value: &Value| -> &Value { &value["bids"] },
            |value: &Value| -> &Value { &value["asks"] },
        );

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.as_ref().unwrap().asks, expected.asks);
        assert_eq!(result.as_ref().unwrap().bids, expected.bids);
    }

    #[rstest]
    #[case(
        1, json ! ( { "bids": [["1.0", "99"]], "asks": [["2.0"]] } ).to_string(), serde::de::Error::custom("Asks error: missing field `amount`".to_string())
    )]
    #[case(
        1, json ! ( { "bids": [["1.0"]], "asks": [["2.0", "100"]] } ).to_string(), serde::de::Error::custom("Bids error: missing field `amount`".to_string())
    )]
    #[case(
        1, json ! ( { "bids": [["1.0"]], "asks": [["2.0"]] } ).to_string(), serde::de::Error::custom("Bids error: missing field `amount`, Asks error: missing field `amount`".to_string())
    )]
    fn deserialize_fails(
        #[case] depth: usize,
        #[case] msg: String,
        #[case] expected: serde_json::Error,
    ) {
        let result = deserialize(
            depth,
            &msg,
            |value: &Value| -> &Value { &value["bids"] },
            |value: &Value| -> &Value { &value["asks"] },
        );

        assert_eq!(result.as_ref().is_err(), true);
        assert_eq!(
            result.as_ref().err().unwrap().to_string(),
            expected.to_string()
        );
    }
}
