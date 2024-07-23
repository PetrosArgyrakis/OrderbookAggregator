# Orderbook Aggregator

![](https://github.com/PetrosArgyrakis/OrderbookAggregator/blob/Prototype/banner.gif)

The service aggregates orderbook snapshots for one symbol, from multiple exchanges and streams the combined orderbook to the clients.

Currently two exchanges are supported:
* Binance
* Bitstamp

### Project structure:

* **client** - the grpc client that prints the aggregated orderbook (executable)
* **data_models** - models (structs) that are shared in the project workspaces (library)
* **exchange_client** - the client adapters that connect to cryptocurrency exchanges (library)
* **orderbook** - orderbook snapshot aggregator implementations (library)
* **server** - the grpc streaming server (executable)

### Usage

Execute ```cargo build``` to build the project.
This will produce 2 executables, one for the server and one for the client.  
The executables ```grpc_server``` and ```grpc_client``` can be found under the /targer directory.  
Execute ```grpc_server --help``` to view the available cli arguments.

Example usage:

```grpc_server --symbol ethbtc``` will subcribe to orderbook snapshots for symbol ```ethbtc```
and will stream the aggregated orderbook to any clients that connect on the default address (```[::1]:50051```).

```grpc_client``` will connect to the server on the default address (```http://[::1]:50051```), will stream the
aggregated orderbook and will print it on the cli.

### Documentation

To build the documentation for this project execute ```cargo doc --no-deps --document-private-items```.

### Performace profiling

This is in progress. The server will be profiled with Intel VTune as a mean to identify any performance bottlenecks. 
