//! The provider that starts a new Grpc server.
//! This is the main entry point for any caller that wants to start the server.


use std::net::ToSocketAddrs;

use tokio::sync::mpsc::UnboundedReceiver;
use tonic::transport::Server;

use data_models::exchange_orderbook::OrderbookSnapshot;

use super::grpc_server::Grpc;
use super::grpc_server::grpc_orderbook::orderbook_aggregator_server::OrderbookAggregatorServer;

pub mod grpc_orderbook {
    tonic::include_proto!("orderbook");
}

/// Starts the Grpc server
///
/// # Arguments
///
/// * `receiver` - The [`UnboundedReceiver<OrderbookSnapshot>`] that receives the orderbook updates from
/// exchange clients.
/// * `addr` - The address of the server.
pub async fn start(
    receiver: UnboundedReceiver<OrderbookSnapshot>,
    addr: &str,
) -> Result<(), tonic::transport::Error> {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();

    Server::builder()
        .add_service(OrderbookAggregatorServer::new(Grpc::new(receiver)))
        .serve(addr)
        .await
}
