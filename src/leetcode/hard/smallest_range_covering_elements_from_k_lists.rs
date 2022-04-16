crate::leetcode::solution!();

use std::collections::{BTreeMap, VecDeque};

impl Solution {
    fn optional_smallest_range(nums: Vec<Vec<i32>>) -> Option<Vec<i32>> {
        let mut index_map = BTreeMap::new();
        let mut list_value_iters = Vec::with_capacity(nums.len());

        for (idx, list) in nums.iter().enumerate() {
            index_map
                .entry(list[0])
                .or_insert_with(|| VecDeque::with_capacity(nums.len()))
                .push_back(idx);

            list_value_iters.push(list.iter());
        }

        let mut min_range = Range(*index_map.keys().next()?, *index_map.keys().next_back()?);

        loop {
            let first_key = *index_map.keys().next()?;
            let queue = index_map.get_mut(&first_key)?;
            let list_id = queue.pop_front()?;

            if queue.is_empty() {
                index_map.remove(&first_key);
            }

            let next_value = match list_value_iters[list_id].next() {
                Some(&next_value) => next_value,
                None => break,
            };

            index_map
                .entry(next_value)
                .or_insert_with(|| VecDeque::with_capacity(nums.len()))
                .push_back(list_id);

            min_range = min_range.min(Range(
                *index_map.keys().next()?,
                *index_map.keys().next_back()?,
            ));
        }

        Some(min_range.into())
    }

    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::optional_smallest_range(nums).unwrap()
    }
}

struct Range(i32, i32);

impl Range {
    fn size(&self) -> i32 {
        self.1 - self.0
    }
}

impl std::cmp::PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.size().eq(&other.size())
    }
}

impl std::cmp::Eq for Range {}

impl std::cmp::PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.size().cmp(&other.size()))
    }
}

impl std::cmp::Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<Range> for Vec<i32> {
    fn from(range: Range) -> Self {
        vec![range.0, range.1]
    }
}
