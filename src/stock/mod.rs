use std::ptr::eq;

pub struct SimplifiedStock {
    pub symbol: String,
    pub price: f64,
    pub name: String,
    pub currency: String,
}

impl SimplifiedStock {
    pub fn display(&self) -> () {
        println!("{}\r\n{}{}", self.symbol, self.price, self.currency);
    }
}

pub trait StockSupplier {
    fn get_quotes(&self, symbol: &String) -> SimplifiedStock;
}


#[cfg(test)]
mod test;
