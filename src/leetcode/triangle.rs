pub struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in 1..triangle.len() {
            for j in 0..=i {
                triangle[i][j] += std::cmp::min(
                    triangle[i - 1][j - (j != 0) as usize],
                    triangle[i - 1][j - (j == i) as usize],
                );
            }
        }

        *triangle[triangle.len() - 1].iter().min().unwrap()
    }
}
