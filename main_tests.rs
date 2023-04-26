use futures::{SinkExt, StreamExt};
use std::net::TcpListener;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;

async fn create_mock_exchange(
    addr: &str,
    response: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    let (stream, _) = listener.accept().await?;
    let mut ws_stream = accept_async(TcpStream::from_std(stream)?).await?;

    loop {
        if let Some(_) = ws_stream.next().await {
            ws_stream.send(Message::Text(response.to_string())).await?;
        }
    }
}

#[tokio::test]
async fn integration_test() {
    let binance_mock_addr = "127.0.0.1:9443";
    let bitstamp_mock_addr = "127.0.0.1:9444";

    let binance_mock_data = r#"{
        "bids": [
            [1.0, 2.0],
            [3.0, 4.0],
            [5.0, 6.0]
        ],
        "asks": [
            [7.0, 8.0],
            [9.0, 10.0],
            [11.0, 12.0]
        ]
    }"#;

    let bitstamp_mock_data = r#"{
        "bids": [
            [1.1, 2.1],
            [3.1, 4.1],
            [5.1, 6.1]
        ],
        "asks": [
            [7.1, 8.1],
            [9.1, 10.1],
            [11.1, 12.1]
        ]
    }"#;

    let binance_mock_handle = tokio::spawn(async move {
        create_mock_exchange(binance_mock_addr, binance_mock_data).await.unwrap();
    });

    let bitstamp_mock_handle = tokio::spawn(async move {
        create_mock_exchange(bitstamp_mock_addr, bitstamp_mock_data).await.unwrap();
    });

    // Wait for the mock servers to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    let (tx, mut rx) = mpsc::unbounded_channel();
    let binance_tx = tx.clone();
    let bitstamp_tx = tx.clone();

    let binance_url = format!("ws://{}", binance_mock_addr);
    let bitstamp_url = format!("ws://{}", bitstamp_mock_addr);

    let binance_handle = tokio::spawn(async move {
        handle_order_book_stream(&binance_url, "binance", binance_tx)
            .await
            .unwrap();
    });

    let bitstamp_handle = tokio::spawn(async move {
        handle_order_book_stream(&bitstamp_url, "bitstamp", bitstamp_tx)
            .await
            .unwrap();
    });

    let received = rx.recv().await.unwrap();
    assert_eq!(received.0, 6.0); // Spread
    assert_eq!(received.1.len(), 6); //
}