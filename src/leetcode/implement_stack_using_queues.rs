use std::collections::LinkedList;

#[allow(dead_code)]
struct MyStack(LinkedList<i32>);

impl MyStack {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(LinkedList::new())
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.0.push_back(x);
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        let stack_size = self.0.len();
        for _ in 1..stack_size {
            let elm = self.0.pop_front().unwrap();
            self.0.push_back(elm);
        }

        self.0.pop_front().unwrap()
    }

    #[allow(dead_code)]
    fn top(&mut self) -> i32 {
        let stack_size = self.0.len();
        for _ in 1..stack_size {
            let elm = self.0.pop_front().unwrap();
            self.0.push_back(elm);
        }

        let top_elm = self.0.pop_front().unwrap();
        self.0.push_back(top_elm);
        top_elm
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.0.is_empty()
    }
}
