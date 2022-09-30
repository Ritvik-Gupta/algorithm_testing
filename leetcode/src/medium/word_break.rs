crate::solution!();

impl Solution {
    pub fn word_break(word: String, dictionary: Vec<String>) -> bool {
        let mut dp = vec![false; word.len() + 1];
        dp[0] = true;

        for i in 1..=word.len() {
            if !dp[i - 1] {
                continue;
            }

            dictionary
                .iter()
                .filter(|dict_word| i - 1 + dict_word.len() <= word.len())
                .for_each(|dict_word| {
                    if &word[i - 1..i - 1 + dict_word.len()] == dict_word {
                        dp[i - 1 + dict_word.len()] = true;
                    }
                });
        }

        *dp.last().unwrap()
    }
}
