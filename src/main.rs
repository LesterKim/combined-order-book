mod proto {
    include!(concat!(env!("OUT_DIR"), "/orderbook.rs"));
}
mod order_book;
mod order_book_test;

use async_tungstenite::tokio::connect_async;
use order_book::{calculate_spread, extract_order_book};
use serde_json::Value;
use std::env;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::wrappers::UnboundedReceiverStream;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use proto::{Empty, Level, OrderbookAggregator, Summary};

type CombinedOrderBook = (f64, Vec<Level>, Vec<Level>);

#[derive(Clone)]
struct OrderbookAggregatorService {
    orderbook: Receiver<CombinedOrderBook>,
}

impl OrderbookAggregator for OrderbookAggregatorService {
    fn book_summary(&mut self, ctx: RpcContext<'_>, _req: Empty, sink: UnarySink<Summary>) {
        let orderbook = self.orderbook.clone();
        let f = async move {
            let mut stream = UnboundedReceiverStream::new(orderbook);
            while let Some((spread, bids, asks)) = stream.next().await {
                let summary = Summary {
                    spread,
                    bids: bids.clone(),
                    asks: asks.clone(),
                };
                sink.send((summary, grpcio::WriteFlags::default()))
                    .await
                    .unwrap();
            }
        };
        ctx.spawn(f);
    }
}

async fn main() {
    let env = Arc::new(Environment::new(1));
    let service = OrderbookAggregatorService {
        orderbook: mpsc::unbounded_channel().1,
    };
    let mut server = ServerBuilder::new(env)
        .register_service(proto::create_orderbook_aggregator(service))
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    let _ = server.wait();
}

async fn fetch_order_book(url: &str, exchange: &str, tx: Sender<CombinedOrderBook>) {
    let (mut socket, _) = connect_async(url).await.unwrap();

    while let Some(msg) = socket.next().await {
        let msg = msg.unwrap().into_text().unwrap();
        let order_book = extract_order_book(&msg, exchange);
        let spread = calculate_spread(&order_book);
        tx.send((spread, order_book)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let binance_url = "wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms";
    let bitstamp_url = "wss://ws.bitstamp.net";

    let (tx, rx) = mpsc::unbounded_channel();
    let binance_tx = tx.clone();
    let bitstamp_tx = tx.clone();

    let binance_handle = tokio::spawn(async move {
        handle_order_book_stream(binance_url, "binance", binance_tx)
            .await
            .unwrap();
    });

    let bitstamp_handle = tokio::spawn(async move {
        handle_order_book_stream(bitstamp_url, "bitstamp", bitstamp_tx)
            .await
            .unwrap();
    });

    let grpc_handle = tokio::spawn(async move {
        run_grpc_server(rx).await;
    });

    futures::future::join3(binance_handle, bitstamp_handle, grpc_handle).await;
}

async fn handle_order_book_stream(
    url: &str,
    exchange: &str,
    tx: Sender<CombinedOrderBook>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut socket, _) = connect_async(url).await?;

    while let Some(msg) = socket.next().await {
        let msg = msg?.into_text()?;
        let order_book = extract_order_book(&msg, exchange);
        let spread = calculate_spread(&order_book);
        tx.send((spread, order_book)).await?;
    }

    Ok(())
}
