crate::solution!();

fn recur_hop_houses(n: usize, nums: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
    match n {
        0 => return nums[0],
        _ if n > nums.len() => return 0,
        _ => {}
    }
    if dp[n] != -1 {
        return dp[n];
    }

    let two = nums[n] + recur_hop_houses(n - 2, nums, dp);
    let one = 0 + recur_hop_houses(n - 1, nums, dp);

    dp[n] = i32::max(one, two);
    dp[n]
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        let mut dp = vec![-1; n + 2];
        recur_hop_houses(n, &nums, &mut dp)
    }
}
