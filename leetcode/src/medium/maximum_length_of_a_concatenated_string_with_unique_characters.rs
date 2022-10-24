crate::solution!();

#[derive(Clone, Copy)]
struct CharBitset(u32);

impl CharBitset {
    fn new() -> Self {
        Self(0)
    }

    fn create_from(word: &str) -> Self {
        let mut bitset = Self::new();
        word.chars().for_each(|ch| bitset.add(ch));
        bitset
    }

    fn add(&mut self, ch: char) {
        self.0 |= 1 << (ch as u8 - b'a');
    }

    fn count_bits(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl std::ops::Mul for CharBitset {
    type Output = (CharBitset, CharBitset);

    fn mul(self, other: Self) -> Self::Output {
        (CharBitset(self.0 & other.0), CharBitset(self.0 | other.0))
    }
}

impl Solution {
    pub fn max_length(words: Vec<String>) -> i32 {
        let mut dp = Vec::with_capacity(words.len() + 1);
        dp.push(CharBitset::new());

        let mut res = 0;
        for word_set in words.iter().filter_map(|word| {
            let word_set = CharBitset::create_from(word);
            match word_set.count_bits() == word.len() {
                true => Some(word_set),
                false => None,
            }
        }) {
            for i in 0..dp.len() {
                let (intersection_set, union_set) = word_set * dp[i];
                if intersection_set.count_bits() > 0 {
                    continue;
                }

                dp.push(union_set);
                res = usize::max(res, union_set.count_bits());
            }
        }

        res as i32
    }
}
