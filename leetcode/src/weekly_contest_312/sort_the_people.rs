crate::solution!();

use std::cmp::Reverse;

struct Person {
    name: String,
    height: i32,
}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people: Vec<_> = names
            .into_iter()
            .zip(heights.into_iter())
            .map(|(name, height)| Person { name, height })
            .collect();

        people.sort_by_key(|person| Reverse(person.height));

        people.into_iter().map(|person| person.name).collect()
    }
}
