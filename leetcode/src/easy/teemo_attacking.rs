crate::solution!();

const MAX_DURATION: i32 = 10000001;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let min_from_duration = |delta| std::cmp::min(duration, delta);

        (0..time_series.len())
            .map(|idx| {
                time_series
                    .get(idx + 1)
                    .map(|&next_instant| next_instant - time_series[idx])
                    .unwrap_or(MAX_DURATION)
            })
            .map(min_from_duration)
            .sum()
    }
}
