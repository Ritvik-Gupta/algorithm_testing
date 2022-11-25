crate::solution!();

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        Some(
            nums.iter()
                .fold((i32::MIN, i32::MAX), |(l, s), &n| (l.max(n), s.min(n))),
        )
        .map(|(l, s)| i32::max(0, l - s - 2 * k))
        .unwrap()
    }
}
