use std::collections::BTreeSet;

struct SmallestInfiniteSet(BTreeSet<i32>);

impl SmallestInfiniteSet {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(BTreeSet::from([1]))
    }

    #[allow(dead_code)]
    fn pop_smallest(&mut self) -> i32 {
        let num = *self.0.range(0..).next().unwrap();
        self.0.remove(&num);
        if self.0.is_empty() {
            self.0.insert(num + 1);
        }
        num
    }

    #[allow(dead_code)]
    fn add_back(&mut self, num: i32) {
        if *self.0.range(0..).next_back().unwrap() <= num {
            return;
        }
        self.0.insert(num);
    }
}
