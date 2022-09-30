crate::solution!();

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
    pub fn find_circle_num(adjacency_matrix: Vec<Vec<i32>>) -> i32 {
        let num_cities = adjacency_matrix.len();
        let cities = 0..num_cities;

        let mut clusters = UnionFind::new(num_cities);

        cities
            .clone()
            .flat_map(|i| cities.clone().map(move |j| (i, j)))
            .filter(|&(i, j)| adjacency_matrix[i][j] == 1)
            .for_each(|(i, j)| clusters.union(i, j));

        cities
            .filter(|&vertex| clusters.find_root(vertex) == vertex)
            .count() as i32
    }
}
