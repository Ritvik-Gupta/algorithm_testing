crate::solution!();

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest_inc_subseq = Vec::with_capacity(nums.len() / 10);

        for &num in nums.iter() {
            let idx = match longest_inc_subseq.binary_search(&num) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            if idx >= longest_inc_subseq.len() {
                longest_inc_subseq.push(num);
            } else {
                longest_inc_subseq[idx] = num;
            }
        }

        longest_inc_subseq.len() as i32
    }
}
