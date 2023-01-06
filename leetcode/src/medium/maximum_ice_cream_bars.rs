crate::solution!();

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort();

        costs
            .into_iter()
            .scan(coins, |coins, cost| {
                *coins -= cost;
                Some(*coins)
            })
            .take_while(|&left_coins| left_coins > 0)
            .count() as _
    }
}
