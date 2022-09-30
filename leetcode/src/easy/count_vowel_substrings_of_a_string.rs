crate::solution!();

fn parse_for_vowels(substring: &[u8]) -> Option<bool> {
    let (mut read_a, mut read_e, mut read_i, mut read_o, mut read_u) =
        (false, false, false, false, false);

    for ch in substring {
        match ch {
            b'a' => read_a = true,
            b'e' => read_e = true,
            b'i' => read_i = true,
            b'o' => read_o = true,
            b'u' => read_u = true,
            _ => return None,
        }
    }
    Some(read_a && read_e && read_i && read_o && read_u)
}

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }

        let word = word.as_bytes();
        let mut result = 0;
        for i in 0..=word.len() - 5 {
            for j in i + 5..=word.len() {
                match parse_for_vowels(&word[i..j]) {
                    Some(does_contains_all_vowels) => {
                        if does_contains_all_vowels {
                            result += 1
                        }
                    }
                    None => break,
                }
            }
        }
        result
    }
}
