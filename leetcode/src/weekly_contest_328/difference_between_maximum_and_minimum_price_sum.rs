crate::solution!();

fn recur_max_path_sum(
    node: usize,
    parent_node: usize,
    adj: &Vec<Vec<usize>>,
    price: &Vec<i32>,
    cache: &mut Vec<Vec<i64>>,
) -> i64 {
    if cache[node][parent_node] == -1 {
        cache[node][parent_node] = price[node] as i64
            + adj[node]
                .iter()
                .filter(|&&child_node| child_node != parent_node)
                .map(|&child_node| recur_max_path_sum(child_node, node, adj, price, cache))
                .max()
                .unwrap_or(0);
    }

    cache[node][parent_node]
}

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;

        let mut adj = vec![Vec::new(); n];
        edges.iter().for_each(|edge| {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        });

        let mut cache = vec![vec![-1; n + 1]; n + 1];

        (0..n)
            .filter(|&node| adj[node].len() == 1)
            .map(|node| recur_max_path_sum(node, n, &adj, &price, &mut cache) - price[node] as i64)
            .max()
            .unwrap()
    }
}
