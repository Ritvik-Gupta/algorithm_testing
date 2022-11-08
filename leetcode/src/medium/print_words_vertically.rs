crate::solution!();

impl Solution {
    pub fn print_vertically(words: String) -> Vec<String> {
        let (num_words, max_word_len) = words
            .split_ascii_whitespace()
            .fold((0, 0), |(num_words, max_word_len), word| {
                (num_words + 1, usize::max(max_word_len, word.len()))
            });

        let mut vertical_words = vec![String::with_capacity(num_words); max_word_len];

        words.split_ascii_whitespace().for_each(|word| {
            word.chars()
                .chain(std::iter::repeat(' ').take(max_word_len - word.len()))
                .enumerate()
                .for_each(|(idx, ch)| vertical_words[idx].push(ch));
        });

        vertical_words
            .iter_mut()
            .for_each(|word| *word = word.trim_end().to_string());

        vertical_words
    }
}
