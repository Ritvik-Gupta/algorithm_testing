crate::solution!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut stones_pq = BinaryHeap::with_capacity(piles.len());
        stones_pq.extend(piles);

        let mut i = 0;
        while i < k {
            let mut bucket = stones_pq.pop().unwrap();
            let smaller_bucket = *stones_pq.peek().unwrap_or(&0);

            while bucket >= smaller_bucket && i < k {
                bucket = (bucket + 1) >> 1;
                i += 1;
            }

            stones_pq.push(bucket);
        }

        stones_pq.into_iter().sum()
    }
}
