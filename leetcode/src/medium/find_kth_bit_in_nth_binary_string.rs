crate::solution!();

fn invert(ch: char) -> char {
    if ch == '0' {
        '1'
    } else {
        '0'
    }
}

use std::cmp::Ordering::*;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        let mid_point = 1 << (n - 1);
        match k.cmp(&mid_point) {
            Equal => '1',
            Less => Solution::find_kth_bit(n - 1, k),
            Greater => invert(Solution::find_kth_bit(n - 1, 2 * mid_point - k)),
        }
    }
}
