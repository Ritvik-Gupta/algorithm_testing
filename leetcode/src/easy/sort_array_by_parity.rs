crate::solution!();

const MOD: i32 = 10000;

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut j = 0;

        for i in 0..nums.len() {
            if (nums[i] % MOD) & 1 == 0 {
                nums[j] += MOD * (nums[i] % MOD);
                j += 1;
            }
        }
        for i in 0..nums.len() {
            if (nums[i] % MOD) & 1 == 1 {
                nums[j] += MOD * (nums[i] % MOD);
                j += 1;
            }
        }

        for i in 0..nums.len() {
            nums[i] = nums[i] / MOD;
        }

        nums
    }
}
