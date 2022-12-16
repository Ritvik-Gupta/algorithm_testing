struct MyQueue {
    stack: Vec<i32>,
}

impl MyQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        let mut rev_stack = Vec::with_capacity(self.stack.len() - 1);

        while self.stack.len() > 1 {
            rev_stack.push(self.stack.pop().unwrap());
        }

        let res = self.stack.pop().unwrap();

        while let Some(elm) = rev_stack.pop() {
            self.stack.push(elm);
        }

        res
    }

    #[allow(dead_code)]
    fn peek(&self) -> i32 {
        *self.stack.first().unwrap()
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
