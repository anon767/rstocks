#[cfg(test)]
mod test {
    use crate::stock::SimplifiedStock;

    #[test]
    fn create_stock() {
        let stock = SimplifiedStock {
            symbol: "AAPL".to_string(),
            name: "APPLE Stocks".to_string(),
            price: 10.,
            currency: "EUR".to_string(),
        };
        assert_eq!(stock.name, "APPLE Stocks", "name should exist");
        assert_eq!(stock.symbol, "AAPL", "symbol should exist");
        assert_eq!(stock.currency, "EUR", "currency should exist");
        assert_eq!(stock.price, 10., "price exist");
    }
}