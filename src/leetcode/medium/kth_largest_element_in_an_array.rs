crate::leetcode::solution!();

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);

        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        }

        heap.pop().unwrap().0
    }
}
