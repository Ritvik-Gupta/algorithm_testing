crate::solution!();

fn compute_scope(expr_rdr: &mut impl Iterator<Item = u8>) -> i32 {
    let (mut result, mut digit_buffer, mut num_sign) = (0, 0, 1);

    while let Some(token) = expr_rdr.next() {
        match token {
            b'0'..=b'9' => digit_buffer = digit_buffer * 10 + (token - b'0') as i32,
            b'(' => digit_buffer = compute_scope(expr_rdr),
            b')' => break,
            _ => {
                result += digit_buffer * num_sign;
                digit_buffer = 0;
                num_sign = if token != b'-' { 1 } else { -1 };
            }
        }
    }

    result + digit_buffer * num_sign
}

impl Solution {
    pub fn calculate(expression: String) -> i32 {
        compute_scope(&mut expression.bytes().filter(|ch| ch != &b' '))
    }
}
