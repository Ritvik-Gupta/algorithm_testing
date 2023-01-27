crate::solution!();

use std::collections::HashSet;

fn dfs(word: &str, word_table: &HashSet<&str>) -> bool {
    (1..word.len()).any(|i| {
        let (prefix, suffix) = (&word[..i], &word[i..]);

        (word_table.contains(prefix) && word_table.contains(suffix))
            || (word_table.contains(prefix) && dfs(suffix, word_table))
            || (word_table.contains(suffix) && dfs(prefix, word_table))
    })
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let word_table = words.iter().map(|s| s.as_str()).collect::<HashSet<_>>();

        words
            .iter()
            .filter(|&word| dfs(word, &word_table))
            .cloned()
            .collect()
    }
}
