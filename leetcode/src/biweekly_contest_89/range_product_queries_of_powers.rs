crate::solution!();

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = Vec::new();

        let mut power = 1;
        while power <= n {
            if n & power != 0 {
                powers.push(power);
            }
            power <<= 1;
        }

        queries
            .iter()
            .map(|query| (query[0] as usize, query[1] as usize))
            .map(|(a, b)| {
                powers[a..=b]
                    .iter()
                    .fold(1i128, |acc, &x| (acc * x as i128) % 1000000007) as i32
            })
            .collect()
    }
}
