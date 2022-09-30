crate::solution!();

use std::cmp::Ordering::Equal;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Equal => b[1].cmp(&a[1]),
            x => x,
        });

        let mut num_weak_characters = 0;
        let mut max_defense = properties.last().unwrap()[1];
        for character_defense in properties.iter().map(|x| x[1]).rev() {
            if character_defense < max_defense {
                num_weak_characters += 1;
            }
            max_defense = i32::max(max_defense, character_defense);
        }
        num_weak_characters
    }
}
