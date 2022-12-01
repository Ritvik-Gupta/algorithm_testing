crate::solution!();

fn is_vowel(ch: &char) -> bool {
    "aeiouAEIOU".contains(*ch)
}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let half_offset = s.len() / 2;
        let mut chars = s.chars();

        let mut take_half_str = || {
            (0..half_offset)
                .filter_map(|_| chars.next())
                .filter(is_vowel)
                .count()
        };

        take_half_str() == take_half_str()
    }
}
