crate::solution!();

use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut graph = vec![Vec::new(); n];
        let mut in_degree = vec![0; n];
        let mut ancestors = vec![BTreeSet::new(); n];

        edges.iter().for_each(|edge| {
            graph[edge[0] as usize].push(edge[1] as usize);
            in_degree[edge[1] as usize] += 1;
        });

        let mut queue = VecDeque::new();
        (0..n)
            .filter(|&node_id| in_degree[node_id] == 0)
            .for_each(|node_id| queue.push_back(node_id));

        while let Some(node_id) = queue.pop_front() {
            for &child_id in graph[node_id].iter() {
                in_degree[child_id] -= 1;

                ancestors[child_id].insert(node_id as i32);
                let ptr: *mut _ = &mut ancestors[child_id];
                unsafe { &mut *ptr }.extend(&ancestors[node_id]);

                if in_degree[child_id] == 0 {
                    queue.push_back(child_id);
                }
            }
        }

        ancestors
            .into_iter()
            .map(|ancestors| ancestors.into_iter().collect())
            .collect()
    }
}
