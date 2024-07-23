use data_models::{aggregated_orderbook::AggregatedOrderbook, exchange_orderbook::OrderbookSnapshot};
use std::collections::HashMap;
use data_models::exchange::Exchange;
use data_models::exchange_level::ExchangeLevel;
use data_models::levels::Levels;
use crate::implementation::hashmap_aggregator::Order::{ASCENDING, DESCENDING};

#[derive(Clone)]
enum Order {
    ASCENDING,
    DESCENDING
}

pub struct HashMapAggregator {
    orderbook_snapshots: HashMap<String, HashMap<Exchange, Levels>>
}

impl crate::api::aggregator::OrderbookSnapshotAggregator for HashMapAggregator {
    /// Constructs a new [HashMapAggregator].
    fn new() -> Self {
        HashMapAggregator {
            orderbook_snapshots: HashMap::<String, HashMap<Exchange, Levels>>::new()
        }
    }

    fn on_orderbook_snapshot(&mut self, os: OrderbookSnapshot) -> AggregatedOrderbook {
        let exchange_levels_map = match self.orderbook_snapshots.get_mut(&os.symbol) {
            None => {
                self.orderbook_snapshots.insert(os.symbol.clone(), HashMap::from([(os.exchange, os.levels)]));
                self.orderbook_snapshots.get_mut(&os.symbol).unwrap()
            },
            Some(hash_map) => {
                hash_map.insert(os.exchange, Levels::new(os.levels.bids, os.levels.asks));
                self.orderbook_snapshots.get_mut(&os.symbol).unwrap()
            }
        };

        let mut aggregated_orderbook = aggregate_exchange_levels(exchange_levels_map);
        sort_aggregated_orderbook(&mut aggregated_orderbook);
        aggregated_orderbook
    }
}

fn aggregate_exchange_levels(exchange_levels: &HashMap<Exchange, Levels>) -> AggregatedOrderbook {
    let mut bids = Vec::<ExchangeLevel>::with_capacity(20);
    let mut asks = Vec::<ExchangeLevel>::with_capacity(20);

    Vec::from_iter(exchange_levels.into_iter()).iter().for_each(|(exchange, levels)| {
        bids.append(&mut levels.bids.iter().map(|level| ExchangeLevel::new((*exchange).clone(), level.clone())).collect::<Vec<ExchangeLevel>>());
        asks.append(&mut levels.asks.iter().map(|level| ExchangeLevel::new((*exchange).clone(), level.clone())).collect::<Vec<ExchangeLevel>>());
    });

    AggregatedOrderbook::new(bids, asks)
}

fn sort_aggregated_orderbook(aggregated_orderbook: &mut AggregatedOrderbook) {
    sort_exchange_levels(&mut aggregated_orderbook.bids, DESCENDING, DESCENDING);
    sort_exchange_levels(&mut aggregated_orderbook.asks, ASCENDING, DESCENDING);
}

fn sort_exchange_levels(el: &mut Vec<ExchangeLevel>, price_order: Order, amount_order: Order) {
    el.sort_by(|el1, el2| match (price_order.clone(), amount_order.clone()) {
        (ASCENDING, DESCENDING) => el1.level.price.partial_cmp(&el2.level.price).unwrap().then(el1.level.amount.partial_cmp(&el2.level.amount).unwrap().reverse()),
        (DESCENDING, DESCENDING) => el1.level.price.partial_cmp(&el2.level.price).unwrap().reverse().then(el1.level.amount.partial_cmp(&el2.level.amount).unwrap().reverse()),
        _ => panic!("The provided sorting combination for price and amount is not supported")
    });
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rstest::rstest;
    use data_models::aggregated_orderbook::AggregatedOrderbook;
    use data_models::exchange_level::{Exchange, ExchangeLevel};
    use data_models::exchange_orderbook::OrderbookSnapshot;
    use data_models::level::Level;
    use data_models::levels::Levels;
    use crate::api::aggregator::OrderbookSnapshotAggregator;
    use crate::implementation::hashmap_aggregator::{HashMapAggregator, Order, sort_exchange_levels, aggregate_exchange_levels, sort_aggregated_orderbook};

    #[test]
    fn on_orderbook_snapshot_test() {
        let mut hashmap_aggregator = HashMapAggregator::new();
        let aggregated_orderbook_1 = hashmap_aggregator.on_orderbook_snapshot(OrderbookSnapshot::new(Exchange::Binance, "test-symbol".to_string(),  Levels::new(vec![Level::new(1.0, 100.0), Level::new(2.0, 110.0)], vec![Level::new(10.0, 200.0), Level::new(11.0, 200.0)])));
        let aggregated_orderbook_2 = hashmap_aggregator.on_orderbook_snapshot(OrderbookSnapshot::new(Exchange::Bitstamp, "test-symbol".to_string(), Levels::new(vec![Level::new(1.5, 110.0), Level::new(2.5, 110.0)], vec![Level::new(10.5, 210.0), Level::new(11.5, 210.0)])));
    }

    #[rstest]
    #[case(
        HashMap::from([
        (Exchange::Bitstamp, Levels::new(vec![Level::new(2.0, 102.0)], vec![Level::new(8.0, 108.0)])),
        (Exchange::Binance, Levels::new(vec![Level::new(1.0, 101.0)], vec![Level::new(7.0, 107.0)]))
        ]),
        AggregatedOrderbook::new(
            vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(2.0, 102.0))],
            vec![ExchangeLevel::new(Exchange::Bitstamp, Level::new(8.0, 108.0)), ExchangeLevel::new(Exchange::Binance, Level::new(7.0, 107.0))]
        )
    )]
    fn aggregate_exchange_levels_test(
        #[case] exchange_levels: HashMap<Exchange, Levels>,
        #[case] expected_aggregated_orderbook: AggregatedOrderbook,
    ) {
        let aggregated_orderbook = aggregate_exchange_levels(&exchange_levels);

        assert!(aggregated_orderbook.asks.iter().all(|it| expected_aggregated_orderbook.asks.contains(it)));
        assert!(aggregated_orderbook.bids.iter().all(|it| expected_aggregated_orderbook.bids.contains(it)));
    }

    #[rstest]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0))],
        Order::ASCENDING,
        Order::DESCENDING
    )]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0))],
        Order::ASCENDING,
        Order::DESCENDING
    )]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0)), ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0))],
        Order::ASCENDING,
        Order::DESCENDING
    )]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0))],
        Order::DESCENDING,
        Order::DESCENDING
    )]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 101.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0))],
        Order::DESCENDING,
        Order::DESCENDING
    )]
    #[case(
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0)), ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 101.0))],
        vec![ExchangeLevel::new(Exchange::Binance, Level::new(2.0, 101.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 100.0))],
        Order::DESCENDING,
        Order::DESCENDING
    )]
    fn sort_exchange_levels_test(
        #[case] mut exchange_levels: Vec<ExchangeLevel>,
        #[case] expected_exchange_levels: Vec<ExchangeLevel>,
        #[case] price_order: Order,
        #[case] amount_order: Order,
    ) {
        sort_exchange_levels(&mut exchange_levels, price_order, amount_order);

        assert_eq!(exchange_levels, expected_exchange_levels);
    }

    #[rstest]
    #[case(
        AggregatedOrderbook::new(
            vec![ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 10.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(3.0, 30.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(2.0, 20.0))],
            vec![ExchangeLevel::new(Exchange::Bitstamp, Level::new(10.0, 110.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(12.0, 130.0)), ExchangeLevel::new(Exchange::Binance, Level::new(11.0, 120.0))],
        ),
        AggregatedOrderbook::new(
            vec![ExchangeLevel::new(Exchange::Bitstamp, Level::new(3.0, 30.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(2.0, 20.0)), ExchangeLevel::new(Exchange::Binance, Level::new(1.0, 10.0))],
            vec![ExchangeLevel::new(Exchange::Bitstamp, Level::new(10.0, 110.0)), ExchangeLevel::new(Exchange::Binance, Level::new(11.0, 120.0)), ExchangeLevel::new(Exchange::Bitstamp, Level::new(12.0, 130.0))],
        )
    )]
    fn sort_orderbook_test(
        #[case] mut aggregated_orderbook: AggregatedOrderbook,
        #[case] expected_aggregated_orderbook: AggregatedOrderbook,
    ) {
        sort_aggregated_orderbook(&mut aggregated_orderbook);

        assert_eq!(aggregated_orderbook.bids, expected_aggregated_orderbook.bids);
        assert_eq!(aggregated_orderbook.asks, expected_aggregated_orderbook.asks);
    }
}


