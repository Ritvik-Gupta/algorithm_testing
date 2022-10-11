crate::solution!();

impl Solution {
    pub fn remove_duplicates(word: String) -> String {
        let mut unique_word = String::new();

        word.chars().for_each(|ch| {
            if unique_word.chars().next_back().unwrap_or(' ') == ch {
                unique_word.pop();
            } else {
                unique_word.push(ch);
            }
        });

        unique_word
    }
}
