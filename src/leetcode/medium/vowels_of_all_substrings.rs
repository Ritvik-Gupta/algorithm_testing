crate::leetcode::solution!();

static VOWELS: &str = "aeiou";

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let num_tokens = word.len();
        word.chars()
            .enumerate()
            .filter(|&(_, token)| VOWELS.contains(token))
            .map(|(pos, _)| ((pos + 1) * (num_tokens - pos)) as i64)
            .sum()
    }
}
