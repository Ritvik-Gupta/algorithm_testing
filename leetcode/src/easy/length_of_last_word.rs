crate::solution!();

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(' ').next_back().unwrap().len() as _
    }
}
