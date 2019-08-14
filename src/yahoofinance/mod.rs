extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use core::result;
use std::fs::File;
use std::io::BufReader;

use crate::stock::SimplifiedStock;
use crate::stock::StockSupplier;

pub struct YahoofinanceSupplier {
    url: &'static str,
}

impl Default for YahoofinanceSupplier {
    fn default() -> YahoofinanceSupplier {
        YahoofinanceSupplier {
            url: "https://query1.finance.yahoo.com/v8/finance/chart/",
        }
    }
}

impl StockSupplier for YahoofinanceSupplier {
    fn get_quotes(&self, symbol: &String) -> SimplifiedStock {
        let root: Root = match self.request_data(symbol.to_string()) {
            Ok(root) => root,
            Err(_f) => panic!("couldnt find symbol: {}", symbol),
        };

        SimplifiedStock {
            symbol: symbol.to_string(),
            price: root.chart.result[0].meta.previous_close.clone(),
            name: root.chart.result[0].meta.exchange_name.clone(),
            currency: root.chart.result[0].meta.currency.clone(),
        }
    }
}

impl YahoofinanceSupplier {
    fn get_symbol_url(&self, symbol: String) -> String {
        format!("{base}{symbol}",
                base = self.url,
                symbol = symbol)
    }

    fn request_data(&self, symbol: String) -> std::result::Result<Root, reqwest::Error> {
        match reqwest::get(&self.get_symbol_url(symbol)) {
            Err(e) => Err(e),
            Ok(mut response) => response.json()
        }
    }
    fn write_to_file(root: Root, filename: String) -> result::Result<(), serde_json::Error> {
        let ofile = match File::create(filename) {
            Ok(file) => file,
            Err(_e) => panic!("Couldnt write file")
        };
        serde_json::to_writer(ofile, &root)
    }

    fn read_from_file(filename: String) -> result::Result<Root, serde_json::Error> {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let u = serde_json::from_reader(reader)?;
                Ok(u)
            }
            Err(_f) => panic!("couldnt open file")
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Root {
    chart: Chart,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Chart {
    result: Vec<Result>,
    error: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Result {
    meta: Meta,
    timestamp: Vec<i64>,
    indicators: Indicators,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Meta {
    currency: String,
    symbol: String,
    #[serde(rename = "exchangeName")]
    exchange_name: String,
    #[serde(rename = "instrumentType")]
    instrument_type: String,
    #[serde(rename = "firstTradeDate")]
    first_trade_date: i64,
    #[serde(rename = "regularMarketTime")]
    regular_market_time: i64,
    gmtoffset: i64,
    timezone: String,
    #[serde(rename = "exchangeTimezoneName")]
    exchange_timezone_name: String,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
    #[serde(rename = "chartPreviousClose")]
    chart_previous_close: f64,
    #[serde(rename = "previousClose")]
    previous_close: f64,
    scale: i64,
    #[serde(rename = "priceHint")]
    price_hint: i64,
    #[serde(rename = "currentTradingPeriod")]
    current_trading_period: CurrentTradingPeriod,
    #[serde(rename = "tradingPeriods")]
    trading_periods: Vec<Vec<TradingPeriod>>,
    #[serde(rename = "dataGranularity")]
    data_granularity: String,
    #[serde(rename = "validRanges")]
    valid_ranges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct CurrentTradingPeriod {
    pre: Pre,
    regular: Regular,
    post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Pre {
    timezone: String,
    start: i64,
    end: i64,
    gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Regular {
    timezone: String,
    start: i64,
    end: i64,
    gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Post {
    timezone: String,
    start: i64,
    end: i64,
    gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct TradingPeriod {
    timezone: String,
    start: i64,
    end: i64,
    gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Quote {
    volume: Vec<Option<i64>>,
    low: Vec<Option<f64>>,
    close: Vec<Option<f64>>,
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
}

#[cfg(test)]
mod test;

