crate::solution!();

use std::collections::HashSet;

fn dfs(
    node: usize,
    visited_nodes: &mut HashSet<usize>,
    adjacency_list: &Vec<Vec<usize>>,
    has_apple: &Vec<bool>,
) -> i32 {
    if visited_nodes.contains(&node) {
        return 0;
    }
    visited_nodes.insert(node);

    let secs = adjacency_list[node]
        .iter()
        .map(|&child_node| dfs(child_node, visited_nodes, adjacency_list, has_apple))
        .sum::<i32>();
    match secs > 0 {
        true => secs + 2,
        _ if has_apple[node] => 2,
        _ => 0,
    }
}

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut adjacency_list = vec![Vec::new(); n];

        edges.into_iter().for_each(|edge| {
            adjacency_list[edge[0] as usize].push(edge[1] as usize);
            adjacency_list[edge[1] as usize].push(edge[0] as usize);
        });

        let mut visited_nodes = HashSet::new();

        return i32::max(
            dfs(0, &mut visited_nodes, &adjacency_list, &has_apple) - 2,
            0,
        );
    }
}
