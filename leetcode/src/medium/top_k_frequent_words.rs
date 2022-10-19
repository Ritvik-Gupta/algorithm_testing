crate::solution!();

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;

        let mut word_table = HashMap::new();
        words
            .into_iter()
            .for_each(|word| *word_table.entry(word).or_insert(0) += 1);

        let mut pq = BinaryHeap::new();
        word_table.into_iter().for_each(|(word, freq)| {
            pq.push((Reverse(freq), word));
            if pq.len() > k {
                pq.pop();
            }
        });

        let mut res = Vec::with_capacity(k);
        for _ in 0..k {
            res.insert(0, pq.pop().unwrap().1);
        }
        res
    }
}
