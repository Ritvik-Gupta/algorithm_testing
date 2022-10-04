crate::solution!();

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let (mut total_time, mut curr_max_time) = (0, 0);
        let mut prev_color = ' ';

        for (color, &time_needed_for_color) in colors.chars().zip(needed_time.iter()) {
            if color != prev_color {
                curr_max_time = 0;
            }

            total_time += i32::min(curr_max_time, time_needed_for_color);
            curr_max_time = i32::max(curr_max_time, time_needed_for_color);
            prev_color = color;
        }

        total_time
    }
}
