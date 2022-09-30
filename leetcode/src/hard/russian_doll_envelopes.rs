crate::solution!();

use std::cmp::Ordering::Equal;

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Equal => a[1].cmp(&b[1]),
            ordering => ordering,
        });

        let mut list = Vec::new();

        for num in envelopes.iter().map(|envelope| envelope[0]) {
            let idx = match list.binary_search(&num) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            if idx >= list.len() {
                list.push(num);
            } else {
                list[idx] = num;
            }
        }

        list.len() as i32
    }
}
