crate::solution!();

use std::collections::HashSet;

fn prime_factors_of(mut num: i32) -> HashSet<i32> {
    let mut prime_factors = HashSet::new();
    let mut prime = 2;
    while num > 1 {
        if num % prime == 0 {
            num /= prime;
            prime_factors.insert(prime);
        } else {
            prime += 1;
        }
    }
    prime_factors
}

impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .flat_map(|num| prime_factors_of(num).into_iter())
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
