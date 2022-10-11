crate::solution!();

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut inc_subseq = Vec::with_capacity(3);

        nums.iter().any(|&num| {
            let idx = inc_subseq
                .iter()
                .position(|&x| x >= num)
                .unwrap_or(inc_subseq.len());

            if idx < inc_subseq.len() {
                inc_subseq[idx] = num;
            } else {
                inc_subseq.push(num);
            }

            inc_subseq.len() == 3
        })
    }
}
