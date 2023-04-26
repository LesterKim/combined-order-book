# Order Book Aggregator

This project is a Rust implementation of an order book aggregator that connects to two exchanges' WebSocket feeds (Binance and Bitstamp), pulls order books for a given traded pair of currencies, merges and sorts the order books to create a combined order book, and publishes the spread, top ten bids, and top ten asks as a stream through a gRPC server.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed Rust (https://www.rust-lang.org/tools/install).
- You have a recent version of `cargo` installed.

## Building and Running the Project

To build and run the project, follow these steps:

1. Clone the repository:

```sh
git clone https://github.com/your_username/order-book-aggregator.git
cd order-book-aggregator
```

2. Build the project:
```sh
cargo build
```

3. Run the project:

```sh
cargo run
```

The application will start and connect to the Binance and Bitstamp WebSocket feeds for the ETHBTC trading pair. It will merge and sort the order books, and then stream the spread, top ten bids, and top ten asks through a gRPC server.

To modify the trading pair or use different WebSocket feeds, update the WebSocket URLs in the src/main.rs file accordingly.

## Running Tests
To run the unit and integration tests, use the following command:

```sh
cargo test
```

## Contributing
If you want to contribute to the project, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more information.
