crate::solution!();

impl Solution {
    pub fn check_string(s: String) -> bool {
        let str_till_first_b = s.chars().skip_while(|&ch| ch != 'b');
        for ch in str_till_first_b {
            if ch == 'a' {
                return false;
            }
        }
        true
    }
}
