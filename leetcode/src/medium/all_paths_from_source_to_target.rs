crate::solution!();

fn recur_construct_paths(
    path: &mut Vec<i32>,
    graph: &Vec<Vec<i32>>,
    all_paths: &mut Vec<Vec<i32>>,
) {
    let node = *path.last().unwrap() as usize;

    if node == graph.len() - 1 {
        all_paths.push(path.clone());
    }

    graph[node].iter().for_each(|&next_node| {
        path.push(next_node);
        recur_construct_paths(path, graph, all_paths);
        path.pop();
    });
}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut all_paths = Vec::new();
        recur_construct_paths(&mut vec![0], &graph, &mut all_paths);
        all_paths
    }
}
