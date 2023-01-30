crate::solution!();

use std::collections::VecDeque;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut tribs = VecDeque::from([0, 1, 1]);

        for _ in 3..=n {
            let tn = tribs.iter().sum::<i32>();
            tribs.pop_front();
            tribs.push_back(tn);
        }
        tribs[usize::min(n as usize, 2)]
    }
}
