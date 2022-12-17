crate::solution!();

fn apply_operator(opr: &str, num1: i32, num2: i32) -> i32 {
    match opr {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        tokens
            .iter()
            .fold(Vec::new(), |mut num_stack, token| {
                match token.as_str() {
                    "+" | "-" | "*" | "/" => {
                        let (num2, num1) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
                        num_stack.push(apply_operator(token, num1, num2));
                    }
                    _ => num_stack.push(token.parse().unwrap()),
                };
                num_stack
            })
            .pop()
            .unwrap()
    }
}
