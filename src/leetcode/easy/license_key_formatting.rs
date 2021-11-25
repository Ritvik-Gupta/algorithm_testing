crate::leetcode::solution!();

const DASH: char = '-';

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut result = String::with_capacity(s.len());
        let mut count = 0;
        for token in s
            .chars()
            .filter(|&x| x != DASH)
            .map(|x| x.to_ascii_uppercase())
            .rev()
        {
            if count == k {
                result.push(DASH);
                count = 0;
            }
            result.push(token);
            count += 1;
        }
        result.chars().rev().collect()
    }
}
