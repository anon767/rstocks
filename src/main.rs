use std::borrow::Borrow;
use std::env;

use crate::stock::StockSupplier;
use crate::yahoofinance::YahoofinanceSupplier;

mod stock;
mod yahoofinance;


fn main() {
    let supplier = YahoofinanceSupplier::default();
    env::args()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, symbol)| supplier.get_quotes(symbol.borrow()))
        .for_each(|a| a.display());
}
