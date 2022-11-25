crate::solution!();

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let (mut min_stack, mut sub_sums) = (Vec::new(), vec![0; arr.len()]);
        let mut total_sum = 0;

        arr.iter().enumerate().for_each(|(i, &n)| {
            while min_stack.last().map_or(false, |&j| arr[j] > n) {
                min_stack.pop();
            }

            sub_sums[i] = match min_stack.last() {
                Some(&j) => sub_sums[j] + (i - j) as u64 * n as u64,
                None => (i + 1) as u64 * n as u64,
            };
            total_sum = (total_sum + sub_sums[i]) % 1_000_000_007;

            min_stack.push(i);
        });

        total_sum as i32
    }
}
