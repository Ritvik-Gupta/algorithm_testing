crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let ideas: Vec<_> = ideas
            .into_iter()
            .map(|idea| idea.as_bytes().to_vec())
            .collect();
        let reserved_names: HashSet<_> = ideas.iter().map(|x| x.clone()).collect();

        let mut total_combinations = 0;
        for i in 0..ideas.len() {
            for j in i + 1..ideas.len() {
                let mut name_a = ideas[i].clone();
                name_a[0] = ideas[j][0];

                let mut name_b = ideas[j].clone();
                name_b[0] = ideas[i][0];

                if !reserved_names.contains(&name_a) && !reserved_names.contains(&name_b) {
                    total_combinations += 2;
                }
            }
        }
        total_combinations
    }
}
