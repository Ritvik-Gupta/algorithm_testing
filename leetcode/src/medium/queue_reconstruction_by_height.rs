crate::solution!();

use std::cmp::Ordering::Equal;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| match b[0].cmp(&a[0]) {
            Equal => a[1].cmp(&b[1]),
            order => order,
        });

        let mut result = Vec::with_capacity(people.len());

        people
            .into_iter()
            .for_each(|person| result.insert(person[1] as usize, person));

        result
    }
}
