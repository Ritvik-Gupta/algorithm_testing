crate::solution!();

impl Solution {
    pub fn nth_super_ugly_number(nth: i32, primes: Vec<i32>) -> i32 {
        let (mut ugly_num, mut dp, mut power_indices, mut candidates) = (
            1,
            Vec::with_capacity(nth as usize),
            vec![0; primes.len()],
            vec![1; primes.len()],
        );

        dp.push(1);
        for _ in 1..nth {
            for i in 0..primes.len() {
                if candidates[i] == ugly_num {
                    candidates[i] =
                        i32::checked_mul(dp[power_indices[i]], primes[i]).unwrap_or(i32::MAX);
                    power_indices[i] += 1;
                }
            }
            ugly_num = *candidates.iter().min().unwrap();
            dp.push(ugly_num);
        }

        *dp.last().unwrap()
    }
}
