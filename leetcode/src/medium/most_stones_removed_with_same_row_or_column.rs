crate::solution!();

use std::collections::HashMap;

struct Centrality {
    root: usize,
    rank: usize,
}

struct UnionFind(Vec<Centrality>);

impl UnionFind {
    fn of_size(size: usize) -> Self {
        Self((0..size).map(|i| Centrality { root: i, rank: 1 }).collect())
    }

    fn find_root(&mut self, vertex: usize) -> usize {
        if vertex != self.0[vertex].root {
            self.0[vertex].root = self.find_root(self.0[vertex].root);
        }
        self.0[vertex].root
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find_root(x);
        let mut root_y = self.find_root(y);
        if root_x == root_y {
            return;
        }

        if self.0[root_x].rank < self.0[root_y].rank {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.0[root_y].root = root_x;
        self.0[root_x].rank += self.0[root_y].rank;
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let (mut x_axis_table, mut y_axis_table) = (HashMap::new(), HashMap::new());

        stones.iter().enumerate().for_each(|(idx, loc)| {
            x_axis_table
                .entry(loc[0])
                .or_insert_with(Vec::new)
                .push(idx);
            y_axis_table
                .entry(loc[1])
                .or_insert_with(Vec::new)
                .push(idx);
        });

        let mut clusters = UnionFind::of_size(stones.len());

        x_axis_table
            .values()
            .chain(y_axis_table.values())
            .for_each(|vertices| {
                vertices
                    .iter()
                    .for_each(|&vertex| clusters.union(vertices[0], vertex));
            });

        (0..clusters.0.len())
            .map(|vertex| match vertex == clusters.find_root(vertex) {
                true => clusters.0[vertex].rank - 1,
                _ => 0,
            })
            .sum::<usize>() as i32
    }
}
