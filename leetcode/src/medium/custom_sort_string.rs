crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

fn make_freq_table(word: &str) -> [usize; 26] {
    let mut freq_table = [0; 26];
    word.chars().for_each(|ch| freq_table[ch_to_idx(ch)] += 1);
    freq_table
}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut freq_table = make_freq_table(&s);
        let mut sorted_word = String::with_capacity(s.len());

        sorted_word.extend(order.chars().flat_map(|ch| {
            let freq = std::mem::take(&mut freq_table[ch_to_idx(ch)]);
            std::iter::repeat(ch).take(freq)
        }));

        sorted_word.extend(
            freq_table
                .iter()
                .enumerate()
                .flat_map(|(i, &freq)| std::iter::repeat((b'a' + i as u8) as char).take(freq)),
        );

        sorted_word
    }
}
