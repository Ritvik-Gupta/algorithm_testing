crate::solution!();

use std::collections::{HashSet, VecDeque};

const STARTING_NODE_ID: usize = 0;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let num_nodes = patience.len();

        let mut adjacency_list = vec![Vec::new(); num_nodes];
        for path_link in edges {
            adjacency_list[path_link[0] as usize].push(path_link[1] as usize);
            adjacency_list[path_link[1] as usize].push(path_link[0] as usize);
        }

        let mut is_visited = HashSet::with_capacity(num_nodes);
        let mut node_queue = VecDeque::with_capacity(num_nodes);

        node_queue.push_back((STARTING_NODE_ID, 0));
        let mut max_delay = 0;

        while let Some((node_id, secs_away)) = node_queue.pop_front() {
            let patience = patience[node_id];
            let loop_back_time = 2 * secs_away;
            if patience > 0 {
                max_delay = max_delay.max(
                    loop_back_time + {
                        let resend = (loop_back_time - 1) / patience;
                        let delay = resend * patience;
                        delay
                    },
                );
            }

            for &link_to_id in adjacency_list[node_id].iter() {
                if is_visited.contains(&link_to_id) {
                    continue;
                }
                node_queue.push_back((link_to_id, secs_away + 1));
                is_visited.insert(link_to_id);
            }
        }
        max_delay + 1
    }
}
