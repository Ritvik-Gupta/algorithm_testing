crate::solution!();

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let target_num = n as usize;
        let square_limit = (n as f64).sqrt().floor() as usize;

        let mut dp: Vec<_> = (0..=n).collect();

        (1..=square_limit).map(|x| x.pow(2)).for_each(|square_val| {
            (square_val..=target_num)
                .for_each(|num| dp[num] = i32::min(dp[num], 1 + dp[num - square_val]));
        });

        dp[target_num]
    }
}
