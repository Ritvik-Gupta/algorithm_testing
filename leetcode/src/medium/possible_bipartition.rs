crate::solution!();

fn dfs(graph: &Vec<Vec<usize>>, colors: &mut Vec<i8>, node: usize, shirt_color: i8) -> bool {
    colors[node] = shirt_color;

    for &adj_node in graph[node].iter() {
        if colors[adj_node] == shirt_color
            || (colors[adj_node] == 0 && !dfs(graph, colors, adj_node, shirt_color * -1))
        {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n + 1];

        dislikes.into_iter().for_each(|dislike| {
            graph[dislike[0] as usize].push(dislike[1] as usize);
            graph[dislike[1] as usize].push(dislike[0] as usize);
        });

        let mut colors = vec![0; n + 1];

        for i in 1..=n {
            if colors[i] == 0 && !dfs(&graph, &mut colors, i, 1) {
                return false;
            }
        }
        return true;
    }
}
