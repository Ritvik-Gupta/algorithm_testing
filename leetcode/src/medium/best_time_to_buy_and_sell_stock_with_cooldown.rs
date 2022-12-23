crate::solution!();

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut b1 = -prices[0];
        let (mut s0, mut s1, mut s2) = (0, 0, 0);

        for i in 1..prices.len() {
            let b0 = i32::max(b1, s2 - prices[i]);
            s0 = i32::max(s1, b1 + prices[i]);
            b1 = b0;
            s2 = s1;
            s1 = s0;
        }
        s0
    }
}
