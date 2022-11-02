crate::solution!();

impl Solution {
    pub fn sort_sentence(sentence: String) -> String {
        let mut words = sentence
            .split(' ')
            .map(|word| {
                (
                    &word[..word.len() - 1],
                    word[word.len() - 1..].parse::<u8>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        words.sort_by_key(|&(_, original_idx)| original_idx);

        let mut res: String = words
            .into_iter()
            .map(|(word, _)| format!("{} ", word))
            .collect();
        res.pop();
        res
    }
}
