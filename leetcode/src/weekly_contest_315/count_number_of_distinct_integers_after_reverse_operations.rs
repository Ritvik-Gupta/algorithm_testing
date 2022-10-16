crate::solution!();

use std::collections::HashSet;

fn reverse(mut num: i32) -> i32 {
    let mut rev_num = 0;
    while num > 0 {
        rev_num = rev_num * 10 + num % 10;
        num /= 10;
    }
    rev_num
}

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut distinct_nums = HashSet::new();

        for num in nums {
            distinct_nums.insert(num);
            distinct_nums.insert(reverse(num));
        }

        distinct_nums.len() as i32
    }
}
