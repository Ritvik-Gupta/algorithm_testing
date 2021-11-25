crate::leetcode::solution!();

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let mut frequencies = BTreeMap::new();
        arr.into_iter()
            .for_each(|token| *frequencies.entry(token).or_insert(0) += 1);

        frequencies
            .into_iter()
            .filter(|(token, frequency)| token == frequency)
            .map(|(token, _)| token)
            .last()
            .unwrap_or(-1)
    }
}
