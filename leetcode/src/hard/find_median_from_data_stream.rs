use std::{
    cmp::{Ordering::*, Reverse},
    collections::BinaryHeap,
    num::Wrapping as Wrapper,
};

const OUT_OF_BOUNDS_NUM: i32 = 100001;

struct MedianFinder {
    max_heap: BinaryHeap<Wrapper<i32>>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    #[allow(dead_code)]
    fn add_num(&mut self, num: i32) {
        match num
            <= self
                .min_heap
                .peek()
                .map_or_else(|| OUT_OF_BOUNDS_NUM, |x| x.0)
        {
            true => self.max_heap.push(Wrapper(num)),
            false => self.min_heap.push(Reverse(num)),
        }

        if self.max_heap.len() == self.min_heap.len() + 2 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap().0));
        } else if self.min_heap.len() == self.max_heap.len() + 2 {
            self.max_heap.push(Wrapper(self.min_heap.pop().unwrap().0));
        }
    }

    #[allow(dead_code)]
    fn find_median(&self) -> f64 {
        let (left, right) = (
            || self.max_heap.peek().unwrap().0,
            || self.min_heap.peek().unwrap().0,
        );

        match self.max_heap.len().cmp(&self.min_heap.len()) {
            Equal => (left() + right()) as f64 / 2.0,
            Greater => left() as f64,
            Less => right() as f64,
        }
    }
}
