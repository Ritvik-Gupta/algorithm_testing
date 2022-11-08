crate::solution!();

impl Solution {
    pub fn make_good(word: String) -> String {
        let mut token_stack = Vec::new();

        word.chars().for_each(|ch| {
            let last_token = *token_stack.last().unwrap_or(&' ');
            if last_token.is_ascii_lowercase() != ch.is_ascii_lowercase()
                && last_token.to_ascii_lowercase() == ch.to_ascii_lowercase()
            {
                token_stack.pop();
            } else {
                token_stack.push(ch);
            }
        });

        token_stack.into_iter().collect()
    }
}
