crate::solution!();

struct AlphabeticFreq([u32; 26]);

impl AlphabeticFreq {
    fn new() -> Self {
        Self([0; 26])
    }

    fn build_for(chars: std::str::Chars) -> Self {
        let mut store = Self::new();
        chars.for_each(|ch| *store.at(ch as u8) += 1);
        store
    }

    fn are_equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn at(&mut self, index: u8) -> &mut u32 {
        &mut self.0[(index - b'a') as usize]
    }
}

struct StrPtr(*const u8);

impl StrPtr {
    fn build(str: &String) -> Self {
        Self(str.as_ptr())
    }

    fn at(&self, index: usize) -> u8 {
        unsafe { *self.0.wrapping_offset(index as isize) }
    }
}

impl Solution {
    pub fn find_anagrams(str: String, pattern: String) -> Vec<i32> {
        let pattern_freq = AlphabeticFreq::build_for(pattern.chars());
        let mut window_freq = AlphabeticFreq::new();
        let mut result = Vec::new();

        let str_ptr = StrPtr::build(&str);
        for idx in 0..=str.len() {
            if idx >= pattern.len() {
                if AlphabeticFreq::are_equal(&pattern_freq, &window_freq) {
                    result.push((idx - pattern.len()) as i32);
                }

                *window_freq.at(str_ptr.at(idx - pattern.len())) -= 1;
            }
            if idx < str.len() {
                *window_freq.at(str_ptr.at(idx)) += 1;
            }
        }

        result
    }
}
