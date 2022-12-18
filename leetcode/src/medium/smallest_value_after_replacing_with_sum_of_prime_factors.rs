crate::solution!();

fn factor_sum_of(mut num: i32) -> i32 {
    let mut factor_sum = 0;
    let mut prime = 2;

    while prime <= num {
        if num % prime == 0 {
            factor_sum += prime;
            num /= prime;
            continue;
        }
        prime += 1;
    }
    factor_sum
}

impl Solution {
    pub fn smallest_value(num: i32) -> i32 {
        let factor_sum = factor_sum_of(num);

        if factor_sum == num {
            num
        } else {
            Solution::smallest_value(factor_sum)
        }
    }
}
