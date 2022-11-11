crate::solution!();

fn construct_freq_table(word: &str) -> [usize; 26] {
    let mut freq_table = [0; 26];
    word.chars()
        .for_each(|ch| freq_table[(ch as u8 - b'a') as usize] += 1);
    freq_table
}

impl Solution {
    pub fn find_the_difference(init: String, word: String) -> char {
        (b'a'
            + construct_freq_table(&init)
                .iter()
                .zip(construct_freq_table(&word).iter())
                .position(|(init_freq, word_freq)| init_freq != word_freq)
                .unwrap() as u8) as char
    }
}
