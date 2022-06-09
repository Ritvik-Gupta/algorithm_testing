crate::leetcode::solution!();

use std::cmp::Ordering::*;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut first, mut second) = (0, numbers.len() - 1);

        while first < second {
            match (numbers[first] + numbers[second]).cmp(&target) {
                Equal => return vec![(first + 1) as i32, (second + 1) as i32],
                Less => first += 1,
                Greater => second -= 1,
            }
        }
        unreachable!()
    }
}
