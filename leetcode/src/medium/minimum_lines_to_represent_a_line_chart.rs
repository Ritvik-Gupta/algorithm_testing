crate::solution!();

macro_rules! windows {
    ($iterator: expr) => {{
        let mut iterator = $iterator;
        let first_val = match iterator.next() {
            Some(val) => val,
            None => return 0,
        };

        iterator.scan(first_val, |prev_val, val| {
            let window = (prev_val.clone(), val.clone());
            *prev_val = val;
            Some(window)
        })
    }};
}

impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        stock_prices.sort_by_key(|stock| stock[0]);

        let stock_gradients = windows!(stock_prices.into_iter()).map(|(st_a, st_b)| {
            (
                (st_a[0] as i64 - st_b[0] as i64),
                (st_a[1] as i64 - st_b[1] as i64),
            )
        });

        1 + windows!(stock_gradients)
            .filter(|(a, b)| a.0 * b.1 != b.0 * a.1)
            .count() as i32
    }
}
