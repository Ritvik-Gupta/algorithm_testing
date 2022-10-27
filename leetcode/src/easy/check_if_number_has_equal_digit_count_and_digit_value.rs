crate::solution!();

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let ch_to_idx = |ch| (ch as u8 - b'0') as usize;

        let mut freq_table = [0; 10];
        num.chars()
            .map(ch_to_idx)
            .for_each(|digit| freq_table[digit] += 1);

        num.char_indices()
            .all(|(digit_idx, ch)| freq_table[digit_idx] == ch_to_idx(ch))
    }
}
