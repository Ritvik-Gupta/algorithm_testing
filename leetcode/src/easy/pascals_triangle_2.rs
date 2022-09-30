crate::solution!();

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;

        let mut result = Vec::with_capacity(row_index + 1);

        let (mut numerator, mut denominator) = (1u128, 1u128);
        for i in 0..=row_index {
            result.push((numerator / denominator) as i32);
            numerator *= (row_index - i) as u128;
            denominator *= (i + 1) as u128;
        }

        result
    }
}
