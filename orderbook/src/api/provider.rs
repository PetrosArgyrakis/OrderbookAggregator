pub use crate::api::aggregator::OrderbookSnapshotAggregator;

pub enum AggregatorType {
    HashMapOrderbookAggegator,
}

pub fn get(aggregator: AggregatorType) -> impl OrderbookSnapshotAggregator {
    match aggregator {
        AggregatorType::HashMapOrderbookAggegator => crate::implementation::hashmap_aggregator::HashMapAggregator::new()
    }
}