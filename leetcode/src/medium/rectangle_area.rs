crate::solution!();

use std::cmp::{max, min};

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        fn area(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
            max(x2 - x1, 0) * max(y2 - y1, 0)
        }

        area(ax1, ay1, ax2, ay2) + area(bx1, by1, bx2, by2)
            - area(max(ax1, bx1), max(ay1, by1), min(ax2, bx2), min(ay2, by2))
    }
}
