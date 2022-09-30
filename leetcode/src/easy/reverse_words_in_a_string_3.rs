crate::solution!();

impl Solution {
    pub fn reverse_words(sentence: String) -> String {
        let mut result: String = sentence
            .split(' ')
            .map(|word| {
                word.chars()
                    .rev()
                    .chain(std::iter::repeat(' ').take(1))
                    .collect::<String>()
            })
            .collect();

        result.pop();
        result
    }
}
