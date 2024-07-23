use data_models::aggregated_orderbook::AggregatedOrderbook;
use data_models::exchange_orderbook::OrderbookSnapshot;

pub trait OrderbookSnapshotAggregator {

    fn new() -> Self;

    /// TCalled when any exchange client publishes an orderbook snapshot.
    ///
    /// # Arguments
    ///
    /// * `orderbook` - The [OrderbookSnapshot] that will be merged into the [AggregatedOrderbook].
    ///
    /// Returns the updated [AggregatedOrderbook]. The return value contains
    /// the aggregated orderbooks of one or more exchanges.
    fn on_orderbook_snapshot(&mut self, orderbook_snapshot: OrderbookSnapshot)
                             -> AggregatedOrderbook;
}
