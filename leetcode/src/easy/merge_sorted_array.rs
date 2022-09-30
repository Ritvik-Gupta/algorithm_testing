crate::solution!();

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);

        while i >= 0 && j >= 0 {
            nums1[k as usize] = match (nums1[i as usize], nums2[j as usize]) {
                (x, y) if x < y => {
                    j -= 1;
                    y
                }
                (x, _) => {
                    i -= 1;
                    x
                }
            };
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
    }
}
