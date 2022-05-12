crate::leetcode::solution!();

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xorred = Vec::with_capacity(arr.len() + 1);
        xorred.push(0);
        xorred.extend(arr.iter().scan(0, |cumulative_xor, elm| {
            *cumulative_xor ^= elm;
            Some(*cumulative_xor)
        }));

        queries
            .iter()
            .map(|query| xorred[query[1] as usize + 1] ^ xorred[query[0] as usize])
            .collect()
    }
}
