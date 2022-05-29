crate::leetcode::solution!();

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let word_bit_masks: Vec<usize> = words
            .iter()
            .map(|word| {
                word.chars()
                    .fold(0, |acc, ch| acc | 1 << (ch as u8 - b'a') as usize)
            })
            .collect();

        let mut max_prod = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if word_bit_masks[i] & word_bit_masks[j] == 0 {
                    max_prod = usize::max(max_prod, words[i].len() * words[j].len());
                }
            }
        }
        max_prod as i32
    }
}
