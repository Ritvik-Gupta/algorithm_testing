crate::solution!();

impl Solution {
    pub fn generate(n: i32) -> Vec<Vec<i32>> {
        let num_rows = n as usize;

        let mut result = Vec::with_capacity(num_rows);
        result.push(vec![1]);
        for row_idx in 1..num_rows {
            result.push(Vec::with_capacity(row_idx + 1));
            result[row_idx].push(1);
            for i in 1..row_idx {
                let sum = result[row_idx - 1][i - 1] + result[row_idx - 1][i];
                result[row_idx].push(sum);
            }
            result[row_idx].push(1);
        }
        result
    }
}
