pub struct Solution;

static CACHED_PRIMES: [u8; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

impl Solution {
    pub fn count_ones(mut num: u32) -> u8 {
        let mut result = 0;
        while num != 0 {
            result += 1;
            num &= num - 1;
        }
        result
    }

    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes: std::collections::HashSet<_> = CACHED_PRIMES.iter().map(|&x| x).collect();

        ((left as u32)..=(right as u32))
            .filter(|&num| primes.contains(&Solution::count_ones(num)))
            .count() as i32
    }
}

// 11110100001001000000
// 01111111111111111111
