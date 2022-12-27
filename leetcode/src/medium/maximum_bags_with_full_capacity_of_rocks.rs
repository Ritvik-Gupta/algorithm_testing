crate::solution!();

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut left_space = capacity
            .into_iter()
            .zip(rocks.into_iter())
            .map(|(cap, rks)| cap - rks)
            .collect::<Vec<_>>();
        left_space.sort();

        left_space
            .iter()
            .scan(0, |acc, space| {
                *acc += space;
                Some(*acc)
            })
            .position(|acc| acc > additional_rocks)
            .unwrap_or(left_space.len()) as i32
    }
}
