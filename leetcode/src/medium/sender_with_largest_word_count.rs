crate::solution!();

use std::cmp::Ordering::*;
use std::collections::HashMap;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut word_count_table = HashMap::with_capacity(senders.len());
        word_count_table.insert("".to_string(), 0);

        messages
            .into_iter()
            .zip(senders.into_iter())
            .for_each(|(msg, sender)| {
                *word_count_table.entry(sender).or_insert(0) += msg.split(' ').count();
            });

        let mut largest_count_sender = "".to_string();

        for (sender, &word_count) in word_count_table.iter() {
            let sender = sender.clone();

            match word_count_table[&largest_count_sender].cmp(&word_count) {
                Less => largest_count_sender = sender,
                Equal if largest_count_sender < sender => largest_count_sender = sender,
                _ => {}
            }
        }

        largest_count_sender
    }
}
