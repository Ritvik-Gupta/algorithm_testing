struct StockSpanner {
    stocks: Vec<(i32, i32)>,
}

impl StockSpanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { stocks: Vec::new() }
    }

    #[inline]
    fn recent_stock_date(&self) -> i32 {
        self.stocks.last().map(|stock| stock.1).unwrap_or(0)
    }

    #[allow(dead_code)]
    fn next(&mut self, price: i32) -> i32 {
        let current_date = self.recent_stock_date();

        while let Some(recent_stock) = self.stocks.last() {
            if recent_stock.0 > price {
                break;
            }
            self.stocks.pop();
        }

        let prev_stock_date = self.recent_stock_date();
        self.stocks.push((price, current_date + 1));
        self.recent_stock_date() - prev_stock_date
    }
}
