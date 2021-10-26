pub struct Solution;

use std::collections::BTreeMap;

pub struct Tribonacci {
    cache: BTreeMap<i32, i32>,
}

impl Tribonacci {
    fn new() -> Self {
        let mut cache = BTreeMap::new();
        cache.insert(0, 0);
        cache.insert(1, 1);
        cache.insert(2, 1);
        Tribonacci { cache }
    }

    fn nth(&mut self, n: i32) -> i32 {
        if let Some(&val) = self.cache.get(&n) {
            return val;
        }
        let sum = self.nth(n - 1) + self.nth(n - 2) + self.nth(n - 3);
        self.cache.insert(n, sum);
        sum
    }
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        Tribonacci::new().nth(n)
    }
}
