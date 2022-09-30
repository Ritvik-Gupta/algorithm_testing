crate::solution!();

impl Solution {
    pub fn first_uniq_char(word: String) -> i32 {
        let mut non_repeating_table = [0u8; 26];

        for ch in word.chars().map(|ch| (ch as u8 - b'a') as usize) {
            let bucket = &mut non_repeating_table[ch];
            if *bucket < 2 {
                *bucket += 1;
            }
        }

        for (idx, ch) in word
            .chars()
            .map(|ch| (ch as u8 - b'a') as usize)
            .enumerate()
        {
            if non_repeating_table[ch] == 1 {
                return idx as i32;
            }
        }

        -1
    }
}
