crate::solution!();

#[derive(PartialEq, Default)]
struct AlphabeticFreq([u16; 26]);

impl AlphabeticFreq {
    fn build_for(chars: std::str::Chars) -> Self {
        let mut store = Self::default();
        chars.for_each(|ch| *store.at(ch as u8) += 1);
        store
    }

    fn at(&mut self, index: u8) -> &mut u16 {
        &mut self.0[(index - b'a') as usize]
    }
}

impl Solution {
    pub fn check_inclusion(pattern: String, mut str: String) -> bool {
        let pattern_freq = AlphabeticFreq::build_for(pattern.chars());
        let mut window_freq = AlphabeticFreq::default();

        let str = unsafe { str.as_bytes_mut() };
        for i in 0..=str.len() {
            if i >= pattern.len() {
                if pattern_freq == window_freq {
                    return true;
                }

                *window_freq.at(str[i - pattern.len()]) -= 1;
            }
            if i < str.len() {
                *window_freq.at(str[i]) += 1;
            }
        }
        false
    }
}
