crate::leetcode::solution!();

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let (mut lowercase_check, mut uppercase_check, mut digit_check, mut special_char_check) =
            (false, false, false, false);
        let mut last_char = ' ';

        for ch in password.chars() {
            if ch == last_char {
                return false;
            }
            last_char = ch;

            if ch.is_ascii_lowercase() {
                lowercase_check = true;
            } else if ch.is_ascii_uppercase() {
                uppercase_check = true;
            } else if ch.is_digit(10) {
                digit_check = true;
            } else if "!@#$%^&*()-+".contains(ch) {
                special_char_check = true;
            }
        }

        lowercase_check && uppercase_check && digit_check && special_char_check
    }
}
