crate::solution!();

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let n = tasks.len();
        let mut res = Vec::with_capacity(n);
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, t)| (t[0], t[1], i))
            .collect::<Vec<_>>();
        tasks.sort();

        let mut i = 0;
        let mut task_pq = BinaryHeap::new();
        let mut time = tasks[0].0;
        while res.len() < n {
            while i < n && tasks[i].0 <= time {
                task_pq.push(Reverse((tasks[i].1, tasks[i].2)));
                i += 1;
            }
            if let Some(Reverse(top_task)) = task_pq.pop() {
                let (t_diff, original_idx) = top_task;
                time += t_diff;
                res.push(original_idx as i32);
            } else if i < n {
                time = tasks[i].0;
            }
        }
        res
    }
}
