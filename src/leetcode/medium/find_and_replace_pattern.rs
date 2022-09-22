crate::leetcode::solution!();

fn to_idx(token: char) -> usize {
    (token as u8 - b'a') as usize
}

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| {
                let mut one_to_mapping = [None; 26];
                let mut to_one_mapping = [None; 26];

                pattern
                    .chars()
                    .zip(word.chars())
                    .all(|(pat_token, org_token)| {
                        if to_one_mapping[to_idx(org_token)]
                            .map(|expected_token| pat_token != expected_token)
                            .unwrap_or(false)
                        {
                            return false;
                        }

                        if one_to_mapping[to_idx(pat_token)]
                            .map(|expected_token| org_token != expected_token)
                            .unwrap_or(false)
                        {
                            return false;
                        }

                        one_to_mapping[to_idx(pat_token)] = Some(org_token);
                        to_one_mapping[to_idx(org_token)] = Some(pat_token);
                        true
                    })
            })
            .collect()
    }
}
