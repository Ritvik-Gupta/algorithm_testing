crate::solution!();

use std::collections::HashSet;

fn contains_duplicate(word: &str) -> bool {
    let mut seen_chars = HashSet::with_capacity(26);
    !word.chars().all(|ch| seen_chars.insert(ch))
}

fn compute_diff_chars(word: &str, goal: &str) -> Vec<(char, char)> {
    word.chars()
        .zip(goal.chars())
        .filter(|(ch_w, ch_g)| ch_w != ch_g)
        .collect()
}

impl Solution {
    pub fn buddy_strings(word: String, goal: String) -> bool {
        if word.len() != goal.len() {
            return false;
        }

        let diff_chars = compute_diff_chars(&word, &goal);
        match diff_chars.len() {
            0 => contains_duplicate(&word),
            2 => diff_chars[0].0 == diff_chars[1].1 && diff_chars[0].1 == diff_chars[1].0,
            _ => false,
        }
    }
}
