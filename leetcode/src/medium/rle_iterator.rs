struct RLEIterator {
    encoding: Vec<i32>,
    running_idx: usize,
}

impl RLEIterator {
    #[allow(dead_code)]
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            encoding,
            running_idx: 0,
        }
    }

    #[allow(dead_code)]
    fn next(&mut self, mut n: i32) -> i32 {
        while *self.encoding.get(self.running_idx).unwrap_or(&i32::MAX) < n {
            n -= self.encoding[self.running_idx];
            self.running_idx += 2;
        }
        if self.running_idx >= self.encoding.len() {
            return -1;
        }

        self.encoding[self.running_idx] -= n;
        self.encoding[self.running_idx + 1]
    }
}
