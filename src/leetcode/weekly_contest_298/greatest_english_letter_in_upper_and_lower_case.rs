crate::leetcode::solution!();

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut chars_occurence = [(false, false); 26];

        for ch in s.chars() {
            let char = &mut chars_occurence[(ch.to_ascii_uppercase() as u8 - b'A') as usize];
            match ch.is_lowercase() {
                true => char.1 = true,
                _ => char.0 = true,
            }
        }

        for i in (0..26).rev() {
            let char = &chars_occurence[i];
            if char.0 == true && char.1 == true {
                return format!("{}", (i as u8 + b'A') as char);
            }
        }
        "".to_string()
    }
}
