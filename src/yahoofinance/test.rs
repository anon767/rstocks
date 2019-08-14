#[cfg(test)]
mod test {
    use crate::yahoofinance::YahoofinanceSupplier;

    #[test]
    fn create_yahoofinance() {
        let yahoofinance = YahoofinanceSupplier::default();
        assert_eq!(yahoofinance.url, "https://query1.finance.yahoo.com/v8/finance/chart/")
    }

    #[test]
    fn fetch_json() {
        let yahoofinance = YahoofinanceSupplier::default();
        let root = yahoofinance.request_data("AAPL".to_string());
        assert!(root.is_ok());
        let testdata = YahoofinanceSupplier::read_from_file("./src/yahoofinance/testdata/test.json".to_string()).unwrap();
        assert_eq!(&testdata.chart.result[0].meta.symbol, "AAPL");
    }
}