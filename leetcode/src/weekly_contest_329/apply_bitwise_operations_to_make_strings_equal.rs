crate::solution!();

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s == target || (s.chars().any(|ch| ch == '1') && target.chars().any(|ch| ch == '1'))
    }
}
