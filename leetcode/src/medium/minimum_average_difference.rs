crate::solution!();

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let mut cumulative_sum = 0;
        let cum_nums = nums
            .iter()
            .map(|&n| {
                cumulative_sum += n as i64;
                cumulative_sum
            })
            .collect::<Vec<_>>();

        let n = nums.len() as i64;
        (0..nums.len())
            .fold((i64::MAX, 0), |(min_diff, min_idx), i| {
                let avg_diff = {
                    let s = i as i64;

                    let left_avg = cum_nums[i] / (s + 1);
                    let right_avg = match n - s - 1 {
                        0 => 0,
                        s => (cumulative_sum - cum_nums[i]) / s,
                    };
                    (left_avg - right_avg).abs()
                };

                match min_diff > avg_diff {
                    true => (avg_diff, i),
                    false => (min_diff, min_idx),
                }
            })
            .1 as i32
    }
}
