crate::solution!();

fn construct_freq_table(word: &str) -> u128 {
    /*
    Considering a BitSet of 26 * 4 bits = 104 bits ( < 128 bits ).
    A given alphabet can have atmost 10 frequency and 10 = (1010)b.
    Max frequency for each alphabet fits in 4 bits and there are 26.

    (---- ---- ---- ... ----)b
        ^    ^    ^        ^
        z    y    x        a
    */
    let mut freq_table = 0u128;

    word.chars().for_each(|ch| {
        let alpha_idx = (ch as u8 - b'a') << 2; // alphabet original index * 4
        freq_table += 1 << alpha_idx; // Add 1 to freq of alphabet when shifted to location
    });

    freq_table
}

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut last_word_table = construct_freq_table("");

        words
            .into_iter()
            .filter(|word| {
                let word_table = construct_freq_table(&word);
                let is_anagram = word_table == last_word_table;
                last_word_table = word_table;
                !is_anagram
            })
            .collect()
    }
}
