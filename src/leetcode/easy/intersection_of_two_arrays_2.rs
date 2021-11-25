crate::leetcode::solution!();

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut nums_freq = HashMap::<i32, usize>::new();
        for num in nums1 {
            *nums_freq.entry(num).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        for num in nums2 {
            nums_freq.entry(num).and_modify(|freq| {
                if *freq > 0 {
                    result.push(num);
                    *freq -= 1;
                }
            });
        }

        result
    }
}
