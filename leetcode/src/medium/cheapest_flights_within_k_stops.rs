crate::solution!();

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1e8 as i32; n];
        dp[src as usize] = 0;

        for _ in 0..=k {
            let mut table = dp.clone();
            for edge in flights.iter() {
                table[edge[1] as usize] =
                    i32::min(table[edge[1] as usize], dp[edge[0] as usize] + edge[2]);
            }
            dp = table;
        }
        (dp[dst as usize] != 1e8 as i32)
            .then(|| dp[dst as usize])
            .unwrap_or(-1)
    }
}
