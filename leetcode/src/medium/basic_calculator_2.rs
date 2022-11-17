crate::solution!();

trait Operator {
    fn precedence(&self) -> i32;
    fn apply_on(&self, num1: i32, num2: i32) -> i32;
}

impl Operator for u8 {
    fn precedence(&self) -> i32 {
        match self {
            b'+' => 1,
            b'-' => 1,
            b'*' => 2,
            b'/' => 2,
            _ => unreachable!(),
        }
    }

    fn apply_on(&self, num1: i32, num2: i32) -> i32 {
        match self {
            b'+' => num1 + num2,
            b'-' => num1 - num2,
            b'*' => num1 * num2,
            b'/' => num1 / num2,
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn calculate(expression: String) -> i32 {
        let (mut num_stack, mut opr_stack) = (Vec::new(), Vec::<u8>::new());
        let mut num_buffer = 0;

        for token in expression.bytes().filter(|ch| ch != &b' ') {
            match token {
                b'0'..=b'9' => num_buffer = num_buffer * 10 + (token - b'0') as i32,
                _ => {
                    num_stack.push(num_buffer);
                    num_buffer = 0;

                    let operator = token;

                    while opr_stack.last().map_or(false, |&prev_opr| {
                        prev_opr.precedence() >= operator.precedence()
                    }) {
                        let (num2, num1) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
                        num_stack.push(opr_stack.pop().unwrap().apply_on(num1, num2));
                    }

                    opr_stack.push(operator);
                }
            }
        }

        num_stack.push(num_buffer);
        while let Some(operator) = opr_stack.pop() {
            let (num2, num1) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
            num_stack.push(operator.apply_on(num1, num2));
        }

        num_stack.pop().unwrap()
    }
}
