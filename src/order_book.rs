use crate::Level;
use serde_json::Value;

pub fn extract_order_book(data: &str, exchange: &str) -> Vec<Level> {
    let v: Value = serde_json::from_str(data).unwrap();
    let bids = v["bids"].as_array().unwrap();
    let asks = v["asks"].as_array().unwrap();

    let mut order_book = Vec::new();
    for bid in bids.iter().take(10) {
        let price = bid[0].as_f64().unwrap();
        let amount = bid[1].as_f64().unwrap();
        order_book.push(Level {
            exchange: exchange.to_string(),
            price,
            amount,
        });
    }

    for ask in asks.iter().take(10) {
        let price = ask[0].as_f64().unwrap();
        let amount = ask[1].as_f64().unwrap();
        order_book.push(Level {
            exchange: exchange.to_string(),
            price,
            amount,
        });
    }

    order_book
}

pub fn calculate_spread(order_book: &[Level]) -> f64 {
    let min_ask = order_book
        .iter()
        .filter(|level| level.side == "ask")
        .min_by_key(|level| OrderedFloat(level.price))
        .unwrap();

    let max_bid = order_book
        .iter()
        .filter(|level| level.side == "bid")
        .max_by_key(|level| OrderedFloat(level.price))
        .unwrap();

    min_ask.price - max_bid.price
}
