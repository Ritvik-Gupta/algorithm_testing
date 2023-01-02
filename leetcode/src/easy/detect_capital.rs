crate::solution!();

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut word = word.chars();

        match word.next().unwrap().is_ascii_lowercase() {
            true => word.all(|letter| letter.is_ascii_lowercase()),
            _ => {
                word.clone().all(|letter| letter.is_ascii_lowercase())
                    || word.all(|letter| letter.is_ascii_uppercase())
            }
        }
    }
}
