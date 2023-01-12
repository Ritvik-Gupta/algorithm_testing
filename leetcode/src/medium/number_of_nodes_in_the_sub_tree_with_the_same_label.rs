crate::solution!();

fn recur_subtree_labels(
    node: usize,
    adj: &Vec<Vec<usize>>,
    labels: &[u8],
    res: &mut Vec<i32>,
) -> [i32; 26] {
    let mut freq_table = [0; 26];
    freq_table[(labels[node] - b'a') as usize] += 1;

    adj[node].iter().for_each(|&child_node| {
        let child_table = recur_subtree_labels(child_node, adj, labels, res);
        (0..26).for_each(|i| freq_table[i] += child_table[i]);
    });

    res[node] = freq_table[(labels[node] - b'a') as usize];
    freq_table
}

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;

        let mut adj = vec![Vec::new(); n];
        edges.iter().for_each(|edge| {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adj[a].push(b);
        });

        let mut res = vec![-1; n];
        recur_subtree_labels(0, &adj, &labels.as_bytes(), &mut res);
        res
    }
}
