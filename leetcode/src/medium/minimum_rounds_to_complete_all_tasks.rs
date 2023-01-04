crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut freq_table = HashMap::new();
        tasks
            .into_iter()
            .for_each(|task| *freq_table.entry(task).or_insert(0) += 1);

        freq_table
            .values()
            .try_fold(0, |total_rounds, &diff_task_freq| match diff_task_freq {
                1 => None,
                _ => Some(total_rounds + (diff_task_freq + 2) / 3),
            })
            .unwrap_or(-1)
    }
}
