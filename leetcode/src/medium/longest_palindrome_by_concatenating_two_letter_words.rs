crate::solution!();

fn word_char_indices(word: &str) -> (usize, usize) {
    let word = word.as_bytes();
    ((word[0] - b'a') as usize, (word[1] - b'a') as usize)
}

const NUM_ALPHABETS: usize = 26;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut freq_table = [[0; NUM_ALPHABETS]; NUM_ALPHABETS];

        words.iter().for_each(|word| {
            let (x, y) = word_char_indices(word);
            freq_table[x][y] += 1;
        });

        let mut has_central_group = false;

        2 * (2
            * (0..NUM_ALPHABETS)
                .map(|i| {
                    has_central_group |= freq_table[i][i] & 1 == 1;

                    (i + 1..NUM_ALPHABETS)
                        .map(|j| i32::min(freq_table[i][j], freq_table[j][i]))
                        .sum::<i32>()
                        + freq_table[i][i] / 2
                })
                .sum::<i32>()
            + if has_central_group { 1 } else { 0 })
    }
}
