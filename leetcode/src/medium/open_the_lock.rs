crate::solution!();

use std::collections::{HashSet, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone, Default)]
struct LockCombination([u8; 4]);

impl LockCombination {
    fn derived_combinations(&self) -> Vec<Self> {
        let mut combinations = Vec::new();
        (0..4).for_each(|i| {
            let mut prev_comb = self.clone();
            prev_comb.0[i] = (prev_comb.0[i] + 10 - 1) % 10;
            combinations.push(prev_comb);

            let mut next_comb = self.clone();
            next_comb.0[i] = (next_comb.0[i] + 1) % 10;
            combinations.push(next_comb);
        });
        combinations
    }
}

impl From<String> for LockCombination {
    fn from(comb: String) -> Self {
        let comb = comb.as_bytes();
        LockCombination([
            comb[0] - b'0',
            comb[1] - b'0',
            comb[2] - b'0',
            comb[3] - b'0',
        ])
    }
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target = LockCombination::from(target);

        let mut deadends = deadends
            .into_iter()
            .map(LockCombination::from)
            .collect::<HashSet<_>>();

        if deadends.contains(&LockCombination::default()) {
            return -1;
        }
        deadends.insert(LockCombination::default());

        let mut queue = VecDeque::new();
        queue.push_back(LockCombination::default());

        let mut level = 0;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                size -= 1;
                let combination = queue.pop_front().unwrap();
                if combination == target {
                    return level;
                }

                combination
                    .derived_combinations()
                    .into_iter()
                    .for_each(|comb| {
                        if !deadends.contains(&comb) {
                            queue.push_back(comb.clone());
                            deadends.insert(comb);
                        }
                    });
            }
            level += 1;
        }
        -1
    }
}
