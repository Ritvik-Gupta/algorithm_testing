crate::solution!();

use std::cmp::Ordering::*;

impl Solution {
    pub fn longest_mountain(mut arr: Vec<i32>) -> i32 {
        arr.push(10001);

        let mut climbing_hill = true;
        let mut longest_hill = 0;
        let mut mountain_start = 0;
        let mut mountain_peak = 0;

        for idx in 1..arr.len() {
            match arr[idx].cmp(&arr[idx - 1]) {
                Less if climbing_hill && mountain_peak != mountain_start => {
                    climbing_hill = false;
                }
                Less if !climbing_hill => {}
                Greater if !climbing_hill && mountain_peak != mountain_start => {
                    longest_hill = std::cmp::max(longest_hill, idx - mountain_start);
                    climbing_hill = true;
                    mountain_start = idx - 1;
                    mountain_peak = idx;
                }
                Greater if climbing_hill => {
                    mountain_peak = idx;
                }
                _ => {
                    if !climbing_hill && mountain_peak != mountain_start {
                        longest_hill = std::cmp::max(longest_hill, idx - mountain_start);
                    }
                    climbing_hill = true;
                    mountain_start = idx;
                    mountain_peak = idx;
                }
            }
        }

        longest_hill as i32
    }
}
