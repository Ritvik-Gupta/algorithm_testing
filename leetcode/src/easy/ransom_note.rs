crate::solution!();

#[inline]
fn build_freq_store(chars: impl Iterator<Item = char>) -> [usize; 26] {
    let mut freq_store = [0; 26];
    chars.for_each(|ch| freq_store[(ch as u8 - b'a') as usize] += 1);
    freq_store
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        build_freq_store(magazine.chars())
            .iter()
            .zip(build_freq_store(ransom_note.chars()).iter())
            .all(|(available_ch, used_ch)| available_ch >= used_ch)
    }
}
