crate::solution!();

impl Solution {
    pub fn remove_stars(word: String) -> String {
        let mut unstar_word =
            String::with_capacity(word.len() - word.chars().filter(|&ch| ch == '*').count());

        word.chars().for_each(|ch| match ch {
            '*' => {
                unstar_word.pop();
            }
            _ => unstar_word.push(ch),
        });

        unstar_word
    }
}
