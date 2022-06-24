crate::leetcode::solution!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut retracting_queue = BinaryHeap::new();
        let mut sum = 0;

        target.into_iter().map(|num| num as i64).for_each(|num| {
            retracting_queue.push(num);
            sum += num;
        });

        while let Some(top_num) = retracting_queue.pop() {
            if top_num == 1 {
                break;
            }

            sum -= top_num;
            if sum == 0 || sum >= top_num {
                return false;
            }

            let old_num = top_num % sum;
            if sum != 1 && old_num == 0 {
                return false;
            }
            retracting_queue.push(old_num);
            sum += old_num;
        }
        return true;
    }
}
