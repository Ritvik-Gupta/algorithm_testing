crate::solution!();

impl Solution {
    pub fn remove_outer_parentheses(parens: String) -> String {
        let mut paren_counter = 0;
        let mut result = String::with_capacity(parens.len());

        for paren in parens.chars() {
            match paren {
                '(' => {
                    paren_counter += 1;
                    if paren_counter != 1 {
                        result.push(paren);
                    }
                }
                ')' => {
                    paren_counter -= 1;
                    if paren_counter != 0 {
                        result.push(paren);
                    }
                }
                _ => unreachable!(),
            }
        }

        result
    }
}
