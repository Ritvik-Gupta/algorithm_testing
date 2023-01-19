crate::solution!();

fn traverse_non_restrictive_nodes(
    node: usize,
    parent_node: usize,
    adj: &Vec<Vec<usize>>,
    restricted: &Vec<i32>,
) -> i32 {
    if restricted.contains(&(node as i32)) {
        return 0;
    }

    1 + adj[node]
        .iter()
        .filter(|&&child_node| child_node != parent_node)
        .map(|&child_node| traverse_non_restrictive_nodes(child_node, node, adj, restricted))
        .sum::<i32>()
}

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];

        edges.into_iter().for_each(|edge| {
            let (x, y) = (edge[0] as usize, edge[1] as usize);
            adj[x].push(y);
            adj[y].push(x);
        });

        traverse_non_restrictive_nodes(0, usize::MAX, &adj, &restricted)
    }
}
