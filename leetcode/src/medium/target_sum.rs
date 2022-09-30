crate::solution!();

use std::{ops::Index, ops::IndexMut};

struct ZveTable {
    table: Vec<i32>,
    zero_offset: usize,
}

impl ZveTable {
    fn with_cap(half_capacity: usize) -> Self {
        Self {
            table: vec![0; 2 * half_capacity + 1],
            zero_offset: half_capacity,
        }
    }
}

impl Index<i32> for ZveTable {
    type Output = i32;

    fn index(&self, index: i32) -> &Self::Output {
        &self.table[index as usize + self.zero_offset]
    }
}

impl IndexMut<i32> for ZveTable {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.table[index as usize + self.zero_offset]
    }
}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total = nums.iter().sum();
        if target.abs() > total {
            return 0;
        }

        let mut prev_level_dp = ZveTable::with_cap(total as usize);
        let mut level_dp = ZveTable::with_cap(total as usize);
        prev_level_dp[0] = 1;

        for i in 0..nums.len() {
            for sum in -total..=total {
                if prev_level_dp[sum] > 0 {
                    level_dp[sum + nums[i]] += prev_level_dp[sum];
                    level_dp[sum - nums[i]] += prev_level_dp[sum];
                    prev_level_dp[sum] = 0;
                }
            }
            std::mem::swap(&mut prev_level_dp, &mut level_dp);
        }

        prev_level_dp[target]
    }
}
