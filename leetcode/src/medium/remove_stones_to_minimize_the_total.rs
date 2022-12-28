crate::solution!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut stones_pq = BinaryHeap::with_capacity(piles.len());
        stones_pq.extend(piles);

        for _ in 0..k {
            let stone_bucket = stones_pq.pop().unwrap();
            stones_pq.push((stone_bucket + 1) >> 1);
        }

        stones_pq.into_iter().sum()
    }
}
