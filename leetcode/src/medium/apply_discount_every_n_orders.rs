struct RoundCounter(u16, u16);

impl RoundCounter {
    fn new(max_count: u16) -> Self {
        Self(0, max_count)
    }

    fn increment(&mut self) -> bool {
        self.0 = (self.0 + 1) % self.1;
        self.0 == 0
    }
}

const MAX_PRODUCTS: usize = 201;

struct Cashier {
    customer_counter: RoundCounter,
    discount_percentage: f64,
    price_of_product: [i32; MAX_PRODUCTS],
}

impl Cashier {
    #[allow(dead_code)]
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut price_of_product = [0; MAX_PRODUCTS];
        products
            .into_iter()
            .zip(prices.into_iter())
            .for_each(|(product_id, price)| price_of_product[product_id as usize] = price);
        Self {
            customer_counter: RoundCounter::new(n as u16),
            discount_percentage: 1.0 - discount as f64 / 100.0,
            price_of_product,
        }
    }

    #[allow(dead_code)]
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        product
            .into_iter()
            .zip(amount.into_iter())
            .map(|(product_id, amount)| self.price_of_product[product_id as usize] * amount)
            .sum::<i32>() as f64
            * if self.customer_counter.increment() {
                self.discount_percentage
            } else {
                1.0
            }
    }
}
