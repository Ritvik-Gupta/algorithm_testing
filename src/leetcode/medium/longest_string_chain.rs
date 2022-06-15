crate::leetcode::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut dp = HashMap::new();
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut result = 1;
        for word in words {
            dp.insert(word.clone(), 1);

            for i in 0..word.len() {
                let predecessor_word = word[..i].to_owned() + &word[i + 1..];
                if let Some(&predecessor_chain) = dp.get(&predecessor_word) {
                    dp.insert(word.clone(), predecessor_chain + 1);
                    result = i32::max(result, dp[&word]);
                }
            }
        }
        result
    }
}
