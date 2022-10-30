crate::solution!();

fn words_delta(w1: &str, w2: &str) -> usize {
    w1.chars()
        .zip(w2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|word| {
                dictionary
                    .iter()
                    .any(|dict_word| words_delta(&dict_word, word) <= 2)
            })
            .collect()
    }
}
