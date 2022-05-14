crate::leetcode::solution!();

use std::collections::VecDeque;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let num_nodes = n as usize;
        let starting_node_id = k as usize;

        let mut adjacency_list = vec![Vec::new(); num_nodes + 1];
        for path_link in times {
            adjacency_list[path_link[0] as usize].push((path_link[1] as usize, path_link[2]));
        }

        let mut node_min_delay = vec![i32::MAX; num_nodes + 1];
        let mut node_queue = VecDeque::with_capacity(num_nodes);

        node_queue.push_back((starting_node_id, 0));
        node_min_delay[starting_node_id] = 0;

        while let Some((node_id, delay)) = node_queue.pop_front() {
            for &(link_to_id, time_taken) in adjacency_list[node_id].iter() {
                let new_delay = delay + time_taken;
                if node_min_delay[link_to_id] <= new_delay {
                    continue;
                }
                node_queue.push_back((link_to_id, delay + time_taken));
                node_min_delay[link_to_id] = new_delay;
            }
        }

        let mut max_delay = 0;
        for &delay in node_min_delay.iter().skip(1) {
            if delay == i32::MAX {
                return -1;
            }
            max_delay = i32::max(max_delay, delay);
        }
        max_delay
    }
}
