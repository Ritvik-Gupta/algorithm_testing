crate::solution!();

const MOD: i32 = 1001;

impl Solution {
    pub fn shuffle(mut nums: Vec<i32>, _: i32) -> Vec<i32> {
        let h = nums.len() / 2;

        for i in 0..h {
            nums[2 * i] += MOD * (nums[i] % MOD);
            nums[2 * i + 1] += MOD * (nums[i + h] % MOD);
        }

        for i in 0..nums.len() {
            nums[i] %= MOD;
        }

        nums
    }
}
