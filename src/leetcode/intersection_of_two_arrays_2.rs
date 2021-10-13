pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut nums1_record = HashMap::<i32, usize>::new();
        for num in nums1 {
            *nums1_record.entry(num).or_insert(0) += 1;
        }

        let mut nums2_record = HashMap::<i32, usize>::new();
        for num in nums2 {
            *nums2_record.entry(num).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        nums1_record.into_iter().for_each(|(num, count1)| {
            if let Some(&count2) = nums2_record.get(&num) {
                result.extend(std::iter::repeat(num).take(std::cmp::min(count1, count2)));
            }
        });

        result
    }
}
