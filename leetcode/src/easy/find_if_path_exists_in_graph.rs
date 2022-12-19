crate::solution!();

use std::collections::VecDeque;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;

        let mut adjacency_list = vec![Vec::new(); n];
        edges.into_iter().for_each(|edge| {
            adjacency_list[edge[0] as usize].push(edge[1] as usize);
            adjacency_list[edge[1] as usize].push(edge[0] as usize);
        });

        let mut node_queue = VecDeque::new();
        let mut visited_nodes = vec![false; n];
        node_queue.push_back(source as usize);

        while let Some(node) = node_queue.pop_front() {
            if node == destination {
                return true;
            }
            if visited_nodes[node] {
                continue;
            }
            visited_nodes[node] = true;

            adjacency_list[node]
                .iter()
                .for_each(|&next_node| node_queue.push_back(next_node));
        }
        false
    }
}
