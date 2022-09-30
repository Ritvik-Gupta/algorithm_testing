crate::solution!();

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct UnionFind {
    rank: Vec<usize>,
    root: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            rank: (0..size).collect(),
            root: (0..size).collect(),
        }
    }

    fn find_root(&mut self, vertex: usize) -> usize {
        if vertex == self.root[vertex] {
            return vertex;
        }
        self.root[vertex] = self.find_root(self.root[vertex]);
        self.root[vertex]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find_root(x);
        let mut root_y = self.find_root(y);
        if root_x == root_y {
            return;
        }
        if self.rank[root_x] < self.rank[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.root[root_y] = root_x;
        self.rank[root_x] += self.rank[root_y];
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(str: String, pairs: Vec<Vec<i32>>) -> String {
        let mut clusters = UnionFind::new(str.len());
        pairs
            .into_iter()
            .for_each(|edge| clusters.union(edge[0] as usize, edge[1] as usize));

        let mut cluster_components = HashMap::new();
        str.char_indices().for_each(|(vertex, token)| {
            cluster_components
                .entry(clusters.find_root(vertex))
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(token))
        });

        (0..str.len())
            .map(|vertex| {
                cluster_components
                    .get_mut(&clusters.find_root(vertex))
                    .unwrap()
                    .pop()
                    .unwrap()
                    .0
            })
            .collect()
    }
}
