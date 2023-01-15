crate::solution!();

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

macro_rules! map {
    ($($key: expr => $value: expr),*) => {
        HashMap::from([$(($key, $value)),*])
    };
}

struct UnionFindMap {
    root: Vec<usize>,
    rank: Vec<HashMap<i32, usize>>,
}

impl UnionFindMap {
    fn new(n: usize, vals: &Vec<i32>) -> Self {
        Self {
            root: (0..n).collect(),
            rank: (0..n).map(|i| map! { vals[i] => 1 }).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = self.find(self.root[x]);
        }
        self.root[x]
    }

    fn union_on(&mut self, x: usize, y: usize, vertex: i32) -> (usize, usize) {
        let (root_x, root_y) = (self.find(x), self.find(y));
        let (rank_x, rank_y) = (
            *self.rank[root_x].get(&vertex).unwrap_or(&0),
            *self.rank[root_y].get(&vertex).unwrap_or(&0),
        );

        self.root[root_y] = root_x;
        self.rank[root_x] = map! { vertex => rank_x + rank_y };

        (rank_x, rank_y)
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut edges = edges
            .into_iter()
            .map(|edge| {
                let (x, y) = (edge[0] as usize, edge[1] as usize);
                (Reverse(i32::max(vals[x], vals[y])), x, y)
            })
            .collect::<BinaryHeap<_>>();

        let mut clusters = UnionFindMap::new(n, &vals);

        let mut res = vals.len();
        res += (0..edges.len())
            .map(|_| edges.pop().unwrap())
            .map(|(Reverse(vertex), x, y)| {
                let (rank_x, rank_y) = clusters.union_on(x, y, vertex);
                rank_y * rank_x
            })
            .sum::<usize>();

        res as i32
    }
}
