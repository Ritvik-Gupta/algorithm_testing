crate::solution!();

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let (mut total_sum, mut min_subarr, mut curr_sum) = (0, 0, 0);

        for i in 0..card_points.len() {
            total_sum += card_points[i];
            curr_sum += card_points[i];
            if i < card_points.len() - k {
                min_subarr += card_points[i];
            } else {
                curr_sum -= card_points[i - (card_points.len() - k)];
                min_subarr = i32::min(min_subarr, curr_sum);
            }
        }

        total_sum - min_subarr
    }
}
