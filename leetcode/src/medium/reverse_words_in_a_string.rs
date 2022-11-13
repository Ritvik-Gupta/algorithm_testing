crate::solution!();

impl Solution {
    pub fn reverse_words(sentence: String) -> String {
        sentence
            .split(' ')
            .filter(|word| !word.is_empty())
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
