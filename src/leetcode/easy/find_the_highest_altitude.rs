crate::leetcode::solution!();

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut largest_height = 0;
        let mut hill_height = 0;
        gain.iter().for_each(|&height| {
            hill_height += height;
            if hill_height > largest_height {
                largest_height = hill_height;
            }
        });

        largest_height
    }
}
