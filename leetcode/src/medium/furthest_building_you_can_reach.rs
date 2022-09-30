crate::solution!();

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let ladders = ladders as usize;
        let mut bricks_left = bricks;

        let mut climbed_using_ladder = BinaryHeap::with_capacity(ladders + 1);

        let mut building_reached = 0;
        while building_reached < heights.len() - 1 {
            let height_diff = heights[building_reached + 1] - heights[building_reached];
            if height_diff > 0 {
                climbed_using_ladder.push(Reverse(height_diff));
                if climbed_using_ladder.len() > ladders {
                    bricks_left -= climbed_using_ladder.pop().unwrap().0;
                    if bricks_left < 0 {
                        break;
                    }
                }
            }
            building_reached += 1;
        }

        building_reached as i32
    }
}
