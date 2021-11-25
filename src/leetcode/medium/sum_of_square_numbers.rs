crate::leetcode::solution!();

impl Solution {
    pub fn judge_square_sum(num: i32) -> bool {
        use std::cmp::Ordering;
        let num = num as i64;
        let (mut start, mut end) = (0i64, (num as f64).sqrt().floor() as i64);

        while start <= end {
            match (start * start + end * end).cmp(&num) {
                Ordering::Equal => return true,
                Ordering::Greater => end -= 1,
                Ordering::Less => start += 1,
            }
        }

        false
    }
}
