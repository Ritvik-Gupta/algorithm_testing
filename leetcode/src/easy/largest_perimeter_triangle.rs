crate::solution!();

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        nums.windows(3)
            .rev()
            .find_map(|window| {
                let win_sum = window.iter().sum();
                match win_sum > 2 * window[2] {
                    true => Some(win_sum),
                    false => None,
                }
            })
            .unwrap_or(0)
    }
}
