crate::solution!();

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|pair| pair[1]);

        let (mut chain_len, mut chain_end) = (0, i32::MIN);

        for pair in pairs {
            if pair[0] > chain_end {
                chain_end = pair[1];
                chain_len += 1;
            }
        }
        chain_len
    }
}
