crate::solution!();

impl Solution {
    pub fn is_alien_sorted(mut words: Vec<String>, order: String) -> bool {
        let mut order_table = [b' '; 26];
        order
            .char_indices()
            .for_each(|(i, ch)| order_table[(ch as u8 - b'a') as usize] = i as u8 + b'a');

        words.iter_mut().for_each(|word| {
            unsafe { word.as_bytes_mut() }
                .iter_mut()
                .for_each(|ch| *ch = order_table[(*ch as u8 - b'a') as usize]);
        });

        let mut prev_word = &words[0];
        words.iter().all(|word| {
            let cmp = word >= prev_word;
            prev_word = word;
            cmp
        })
    }
}
