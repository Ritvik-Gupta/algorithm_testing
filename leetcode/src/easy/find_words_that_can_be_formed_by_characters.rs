crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

fn build_freq_store(chars: impl Iterator<Item = char>) -> [usize; 26] {
    let mut freq_store = [0; 26];
    chars.for_each(|ch| freq_store[ch_to_idx(ch)] += 1);
    freq_store
}

impl Solution {
    pub fn count_characters(words: Vec<String>, available_chars: String) -> i32 {
        let available_store = build_freq_store(available_chars.chars());

        words
            .iter()
            .filter(|word| {
                available_store
                    .iter()
                    .zip(build_freq_store(word.chars()).iter())
                    .all(|(available_ch, used_ch)| available_ch >= used_ch)
            })
            .map(|word| word.len())
            .sum::<usize>() as i32
    }
}
