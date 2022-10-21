crate::solution!();

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.iter()
            .scan(std::collections::HashSet::new(), |seen_nums, &num| {
                Some(!seen_nums.insert(num))
            })
            .any(|does_contain_duplicate| does_contain_duplicate)
    }
}
