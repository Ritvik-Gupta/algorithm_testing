crate::solution!();

use std::collections::BinaryHeap;

struct HeapElm(i32, usize, usize);

impl PartialEq for HeapElm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapElm {}

use std::cmp::Ordering;

impl PartialOrd for HeapElm {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for HeapElm {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let n = matrix.len();
        let mut pq = BinaryHeap::with_capacity(n);

        for i in 0..n {
            pq.push(HeapElm(matrix[i][0], i, 0));
        }

        while k > 1 {
            let HeapElm(_, x, y) = pq.pop().unwrap();

            if y + 1 < n {
                pq.push(HeapElm(matrix[x][y + 1], x, y + 1));
            }
            k -= 1;
        }

        pq.pop().unwrap().0
    }
}
