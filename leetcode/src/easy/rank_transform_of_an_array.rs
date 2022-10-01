crate::solution!();

struct RankToken(i32, usize);

impl PartialEq for RankToken {
    fn eq(&self, other: &Self) -> bool {
        other.0 == self.0
    }
}

impl Eq for RankToken {}

impl PartialOrd for RankToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for RankToken {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

const OUT_OF_BOUNDS_VALUE: i32 = -1000000001;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let (mut rank_heap, mut prev_val, mut current_rank) = (
            BinaryHeap::from_iter(arr.iter().enumerate().map(|(pos, &x)| RankToken(x, pos))),
            OUT_OF_BOUNDS_VALUE,
            0,
        );

        while let Some(store) = rank_heap.pop() {
            if store.0 != prev_val {
                current_rank += 1;
            }
            arr[store.1] = current_rank;
            prev_val = store.0;
        }
        arr
    }
}
