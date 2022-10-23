crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'A') as usize
}

impl Solution {
    pub fn min_window(word: String, ncsry_word: String) -> String {
        let mut ncsry_char_freq = [0; 58];
        ncsry_word
            .chars()
            .for_each(|ch| ncsry_char_freq[ch_to_idx(ch)] += 1);
        let ncsry_size = ncsry_char_freq
            .iter()
            .filter(|&&ncsry_ch| ncsry_ch > 0)
            .count() as i32;

        let filtered_word_indices = word
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ncsry_char_freq[ch_to_idx(ch)] > 0)
            .collect::<Vec<_>>();

        let (mut min_size, mut min_srt, mut min_end) = (usize::MAX, 0, 0);

        let mut char_freq = [0; 58];
        let mut l = 0;
        let mut formed_size = 0;
        for r in 0..filtered_word_indices.len() {
            let (end, ch) = filtered_word_indices[r];
            char_freq[ch_to_idx(ch)] += 1;

            if char_freq[ch_to_idx(ch)] == ncsry_char_freq[ch_to_idx(ch)] {
                formed_size += 1;
            }

            while l <= r && formed_size == ncsry_size {
                let (srt, ch) = filtered_word_indices[l];
                let size = end - srt + 1;

                if min_size > size {
                    min_size = size;
                    min_srt = srt;
                    min_end = end;
                }

                char_freq[ch_to_idx(ch)] -= 1;
                if char_freq[ch_to_idx(ch)] < ncsry_char_freq[ch_to_idx(ch)] {
                    formed_size -= 1;
                }
                l += 1;
            }
        }

        match min_size {
            usize::MAX => "",
            _ => &word[min_srt..=min_end],
        }
        .to_string()
    }
}
