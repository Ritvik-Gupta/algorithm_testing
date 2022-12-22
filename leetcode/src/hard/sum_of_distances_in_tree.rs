crate::solution!();

fn post_order_dfs(
    node: usize,
    prev_node: usize,
    adjacency_list: &Vec<Vec<usize>>,
    res: &mut Vec<i32>,
    count: &mut Vec<i32>,
) {
    adjacency_list[node]
        .iter()
        .filter(|&&next_node| next_node != prev_node)
        .for_each(|&next_node| {
            post_order_dfs(next_node, node, adjacency_list, res, count);
            count[node] += count[next_node];
            res[node] += res[next_node] + count[next_node];
        });
    count[node] += 1;
}

fn pre_order_dfs(
    node: usize,
    prev_node: usize,
    adjacency_list: &Vec<Vec<usize>>,
    res: &mut Vec<i32>,
    count: &mut Vec<i32>,
) {
    adjacency_list[node]
        .iter()
        .filter(|&&next_node| next_node != prev_node)
        .for_each(|&next_node| {
            res[next_node] = res[node] + count.len() as i32 - 2 * count[next_node];
            pre_order_dfs(next_node, node, adjacency_list, res, count);
        });
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adjacency_list = vec![Vec::new(); n];

        edges.into_iter().for_each(|edge| {
            adjacency_list[edge[0] as usize].push(edge[1] as usize);
            adjacency_list[edge[1] as usize].push(edge[0] as usize);
        });

        let mut res = vec![0; n];
        let mut count = vec![0; n];

        post_order_dfs(0, usize::MAX, &adjacency_list, &mut res, &mut count);
        pre_order_dfs(0, usize::MAX, &adjacency_list, &mut res, &mut count);

        res
    }
}
