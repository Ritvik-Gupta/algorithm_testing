crate::leetcode::solution!();

impl Solution {
    pub fn is_match(word: String, pattern: String) -> bool {
        let space = word.len() + 1;

        let mut partial_match = vec![false; space];
        let mut level_match = vec![false; space];
        level_match[0] = true;

        for match_token in pattern.chars() {
            std::mem::swap(&mut partial_match, &mut level_match);

            if match_token == '*' {
                partial_match
                    .iter_mut()
                    .enumerate()
                    .skip_while(|(_, had_prev_match)| **had_prev_match == false)
                    .for_each(|(i, had_prev_match)| {
                        level_match[i] = true;
                        *had_prev_match = false;
                    });
                continue;
            }

            let mut word_ptr = word.as_ptr();
            for i in 1..space {
                if partial_match[i - 1] {
                    match match_token {
                        '?' => level_match[i] = true,
                        'a'..='z' if match_token == char::from(unsafe { *word_ptr }) => {
                            level_match[i] = true
                        }
                        _ => {}
                    }
                }

                partial_match[i - 1] = false;
                word_ptr = word_ptr.wrapping_offset(1);
            }
            partial_match[space - 1] = false;
        }

        level_match[word.len()]
    }
}
