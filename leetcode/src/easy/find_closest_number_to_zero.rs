crate::solution!();

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .min_by(|elm1, elm2| match elm1.abs().cmp(&elm2.abs()) {
                std::cmp::Ordering::Equal => elm2.cmp(elm1),
                ord => ord,
            })
            .unwrap()
    }
}
