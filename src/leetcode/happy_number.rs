pub struct Solution;

impl Solution {
    fn sum_of_squares(mut num: u32) -> u32 {
        let mut sum = 0;
        while num != 0 {
            let rem = num % 10;
            sum += rem * rem;
            num /= 10;
        }
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        let mut num = n as u32;
        let mut cache = HashSet::new();
        while num != 1 {
            cache.insert(num);
            num = Solution::sum_of_squares(num);
            if cache.contains(&num) {
                return false;
            }
        }
        true
    }
}
