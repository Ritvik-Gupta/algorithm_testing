crate::leetcode::solution!();

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let (mut j, mut s, mut ans) = (0, 0, 0);

        for i in 0..nums.len() {
            s += nums[i];
            while s as i64 * (i - j + 1) as i64 >= k {
                s -= nums[j];
                j += 1;
            }
            ans += (i - j + 1) as i64;
        }
        ans
    }
}
