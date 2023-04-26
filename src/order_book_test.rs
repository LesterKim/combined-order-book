#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn test_extract_order_book() {
        let data = r#"
        {
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
        }
        "#;

        let order_book = extract_order_book(data, "test");
        assert_eq!(order_book.len(), 6);

        let bid = &order_book[0];
        assert_eq!(bid.exchange, "test");
        assert_eq!(bid.price, 1.0);
        assert_eq!(bid.amount, 2.0);

        let ask = &order_book[3];
        assert_eq!(ask.exchange, "test");
        assert_eq!(ask.price, 7.0);
        assert_eq!(ask.amount, 8.0);
    }

    #[tokio::test]
    async fn test_calculate_spread() {
        let order_book = vec![
            Level {
                exchange: "test".to_string(),
                price: 1.0,
                amount: 2.0,
            },
            Level {
                exchange: "test".to_string(),
                price: 3.0,
                amount: 4.0,
            },
            Level {
                exchange: "test".to_string(),
                price: 5.0,
                amount: 6.0,
            },
        ];

        let spread = calculate_spread(&order_book);
        assert_eq!(spread, 4.0);
    }
}
