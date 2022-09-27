crate::leetcode::solution!();

use std::cmp::Ordering::*;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let total_dominoes = dominoes.len() as isize;

        let right_forces = dominoes
            .chars()
            .scan(0, |force, force_orientation| {
                *force = match force_orientation {
                    'R' => total_dominoes,
                    'L' => 0,
                    _ if *force == 0 => 0,
                    _ => *force - 1,
                };

                Some(*force)
            })
            .collect::<Vec<_>>();

        let left_forces = dominoes
            .chars()
            .rev()
            .scan(0, |force, force_orientation| {
                *force = match force_orientation {
                    'L' => total_dominoes,
                    'R' => 0,
                    _ if *force == 0 => 0,
                    _ => *force - 1,
                };

                Some(*force)
            })
            .collect::<Vec<_>>();

        right_forces
            .into_iter()
            .zip(left_forces.into_iter().rev())
            .map(|(right_force, left_force)| right_force - left_force)
            .map(|force_sum| match force_sum.cmp(&0) {
                Less => 'L',
                Greater => 'R',
                Equal => '.',
            })
            .collect()
    }
}
