struct StockNode {
    date: i32,
    price: i32,
    next: Option<Box<StockNode>>,
}

impl StockNode {
    fn new(date: i32, price: i32, next: Option<Box<Self>>) -> Box<Self> {
        Box::new(Self { date, price, next })
    }

    fn get_date(stock: &Option<Box<StockNode>>) -> i32 {
        stock.as_ref().map(|stock| stock.date).unwrap_or(0)
    }
}

struct StockSpanner {
    stock_list: Option<Box<StockNode>>,
}

impl StockSpanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { stock_list: None }
    }

    #[allow(dead_code)]
    fn next(&mut self, price: i32) -> i32 {
        let current_date = StockNode::get_date(&self.stock_list);
        let mut curr_stock = self.stock_list.take();
        loop {
            match curr_stock {
                Some(mut stock) if price >= stock.price => curr_stock = stock.next.take(),
                _ => break,
            }
        }
        let prev_stock_date = StockNode::get_date(&curr_stock);
        self.stock_list = Some(StockNode::new(current_date + 1, price, curr_stock));
        StockNode::get_date(&self.stock_list) - prev_stock_date
    }
}
