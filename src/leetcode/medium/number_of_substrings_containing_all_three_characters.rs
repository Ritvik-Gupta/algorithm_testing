crate::leetcode::solution!();

struct SlidingFreqWindow([u32; 3]);

impl SlidingFreqWindow {
    fn new() -> Self {
        Self([0; 3])
    }

    fn product(&self) -> u32 {
        self.0[0] * self.0[1] * self.0[2]
    }
}

const FREQUENCY_OFFSET: u8 = b'a';

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let (word, mut sliding_freq_window) = (s.as_bytes(), SlidingFreqWindow::new());
        let mut j = 0;

        (0..word.len())
            .map(|i| {
                let mut total_combinations = 0;
                sliding_freq_window.0[(word[i] - FREQUENCY_OFFSET) as usize] += 1;
                while sliding_freq_window.product() != 0 {
                    total_combinations += (word.len() - i) as i32;
                    sliding_freq_window.0[(word[j] - FREQUENCY_OFFSET) as usize] -= 1;
                    j += 1;
                }
                total_combinations
            })
            .sum()
    }
}
