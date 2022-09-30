crate::solution!();

struct UnionFind {
    rank: Vec<usize>,
    root: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            rank: vec![1; size],
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
    pub fn count_pairs(num_nodes: i32, edges: Vec<Vec<i32>>) -> i64 {
        let num_nodes = num_nodes as usize;
        let mut clusters = UnionFind::new(num_nodes);

        edges
            .iter()
            .for_each(|edge| clusters.union(edge[0] as usize, edge[1] as usize));

        let mut cluster_sizes = Vec::new();
        for vertex in 0..num_nodes {
            if clusters.find_root(vertex) == vertex {
                cluster_sizes.push(clusters.rank[vertex] as i64);
            }
        }

        let num_nodes = num_nodes as i64;
        cluster_sizes
            .iter()
            .fold(0, |total_unreachability, cluster_size| {
                total_unreachability + (num_nodes - cluster_size) * cluster_size
            })
            / 2
    }
}
