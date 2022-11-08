crate::solution!();

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut ans, mut sum, mut odd, mut even) = (0i64, 0, 0, 0);
        arr.iter().for_each(|&elm| {
            sum = (sum + elm) % 2;
            if sum == 0 {
                ans += odd;
                even += 1;
            } else {
                ans += 1 + even;
                odd += 1;
            }
        });

        return (ans % 1000000007) as i32;
    }
}
