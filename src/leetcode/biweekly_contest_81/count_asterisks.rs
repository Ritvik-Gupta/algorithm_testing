crate::leetcode::solution!();

impl Solution {
    pub fn count_asterisks(str: String) -> i32 {
        let mut is_inside_bar = false;
        str.chars().fold(0, |acc, ch| {
            acc + match ch {
                '*' if !is_inside_bar => 1,
                _ => {
                    if ch == '|' {
                        is_inside_bar = !is_inside_bar;
                    }
                    0
                }
            }
        })
    }
}
