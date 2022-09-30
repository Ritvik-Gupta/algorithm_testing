crate::solution!();

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn eventual_safe_nodes(adjacency_list: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![HashSet::new(); adjacency_list.len()];
        let mut rev_graph = vec![HashSet::new(); adjacency_list.len()];
        let mut safe_nodes = vec![false; adjacency_list.len()];

        let mut terminal_queue = VecDeque::new();
        for (node, connections) in adjacency_list.iter().enumerate() {
            if connections.is_empty() {
                terminal_queue.push_back(node);
            }

            for &conn_node in connections {
                graph[node].insert(conn_node as usize);
                rev_graph[conn_node as usize].insert(node);
            }
        }

        while let Some(terminal_node) = terminal_queue.pop_front() {
            safe_nodes[terminal_node] = true;

            for &rev_conn_node in rev_graph[terminal_node].iter() {
                graph[rev_conn_node].remove(&terminal_node);
                if graph[rev_conn_node].is_empty() {
                    terminal_queue.push_back(rev_conn_node);
                }
            }
        }

        safe_nodes
            .iter()
            .enumerate()
            .filter(|&(_, &is_safe)| is_safe)
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}
