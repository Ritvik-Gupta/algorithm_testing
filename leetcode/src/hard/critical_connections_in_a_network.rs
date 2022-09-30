crate::solution!();

use std::collections::HashSet;

struct Bridges {
    visited: HashSet<usize>,
    disc: Vec<usize>,
    low: Vec<usize>,
    parent: Vec<Option<usize>>,
    time: usize,
}

const INVALID_NODE_ID: usize = 100001;

impl Bridges {
    fn build_graph(num_nodes: usize, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacency_list = vec![Vec::new(); num_nodes];

        for conn in connections {
            adjacency_list[conn[0] as usize].push(conn[1]);
            adjacency_list[conn[1] as usize].push(conn[0]);
        }
        adjacency_list
    }

    fn in_graph(adjacency_list: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let num_nodes = adjacency_list.len();

        let mut bridges_builder = Self {
            visited: HashSet::with_capacity(num_nodes),
            disc: vec![0; num_nodes],
            low: vec![0; num_nodes],
            parent: vec![None; num_nodes],
            time: 0,
        };

        let mut bridges = Vec::new();
        for i in 0..num_nodes {
            if !bridges_builder.visited.contains(&i) {
                bridges_builder.recursive_build_from_node(i, adjacency_list, &mut bridges);
            }
        }
        bridges
    }

    fn recursive_build_from_node(
        &mut self,
        node: usize,
        adjacency_list: &Vec<Vec<i32>>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        self.visited.insert(node);

        self.time += 1;
        self.low[node] = self.time;
        self.disc[node] = self.time;

        for neighbor_node in adjacency_list[node].iter().map(|&x| x as usize) {
            if !self.visited.contains(&neighbor_node) {
                self.parent[neighbor_node] = Some(node);
                self.recursive_build_from_node(neighbor_node, adjacency_list, bridges);

                self.low[node] = usize::min(self.low[node], self.low[neighbor_node]);

                if self.low[neighbor_node] > self.disc[node] {
                    bridges.push(vec![node as i32, neighbor_node as i32]);
                }
            } else if neighbor_node != self.parent[node].unwrap_or(INVALID_NODE_ID) {
                self.low[node] = usize::min(self.low[node], self.disc[neighbor_node]);
            }
        }
    }
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Bridges::in_graph(&Bridges::build_graph(n as usize, connections))
    }
}
