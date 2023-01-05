crate::solution!();

use std::collections::BinaryHeap;

fn dfs_for_tree_path(node: usize, adj_list: &Vec<Vec<usize>>, crown: &[u8], res: &mut i32) -> i32 {
    let mut candidates = BinaryHeap::new();
    candidates.push(0);

    for &child_node in adj_list[node].iter() {
        let curr = dfs_for_tree_path(child_node, adj_list, crown, res);
        if crown[node] != crown[child_node] {
            candidates.push(curr);
        }
    }

    let (a, b) = (candidates.pop().unwrap_or(0), candidates.pop().unwrap_or(0));
    *res = i32::max(*res, a + b + 1);
    a + 1
}

impl Solution {
    pub fn longest_path(parent: Vec<i32>, crown: String) -> i32 {
        let mut adj_list = vec![Vec::new(); crown.len()];
        parent[1..]
            .into_iter()
            .zip(1..)
            .for_each(|(&parent_node, node)| adj_list[parent_node as usize].push(node));

        let mut res = 0;
        dfs_for_tree_path(0, &adj_list, crown.as_bytes(), &mut res);
        res
    }
}
