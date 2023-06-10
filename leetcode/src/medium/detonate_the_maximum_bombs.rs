crate::solution!();

use std::collections::{HashMap, HashSet};

macro_rules! destructure {
    ($vec: expr => $($val: tt),+ as $type: ty) => {
        let mut _counter = 0;
        $(
            let $val = $vec[_counter] as $type;
            _counter += 1;
        )+
    };
}

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let (n, mut graph) = (bombs.len(), HashMap::new());

        for i in 0..n {
            graph.insert(i, Vec::new());
            for j in 0..n {
                destructure!(bombs[i] => xi, yi, ri as i64);
                destructure!(bombs[j] => xj, yj as i64);

                if i != j && (xi - xj).pow(2) + (yi - yj).pow(2) <= ri.pow(2) {
                    graph.get_mut(&i).unwrap().push(j);
                }
            }
        }

        fn dfs(node: usize, visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>) {
            graph[&node].iter().for_each(|&child| {
                if !visited.contains(&child) {
                    visited.insert(child);
                    dfs(child, visited, graph)
                }
            })
        }

        let mut visited = HashSet::new();
        (0..n)
            .map(|i| {
                visited.clear();
                visited.insert(i);
                dfs(i, &mut visited, &graph);
                visited.len()
            })
            .max()
            .unwrap() as i32
    }
}
