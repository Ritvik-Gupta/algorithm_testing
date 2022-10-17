crate::solution!();

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let highest_candies = *candies.iter().max().unwrap();
        candies
            .iter()
            .map(|&candy| extra_candies + candy >= highest_candies)
            .collect()
    }
}
