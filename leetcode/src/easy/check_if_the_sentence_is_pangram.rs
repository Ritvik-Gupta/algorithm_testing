crate::solution!();

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut char_store = 0u32;
        sentence
            .chars()
            .for_each(|ch| char_store |= 1 << (ch as u8 - b'a') as usize);

        char_store + 1 == 1 << 26
    }
}
