crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as _
}

struct UnionFind {
    root: Vec<usize>,
}

impl UnionFind {
    fn new() -> Self {
        Self {
            root: (0..26).collect(),
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_root(x);
        let root_y = self.find_root(y);

        if root_x > root_y {
            self.root[root_x] = root_y;
        } else {
            self.root[root_y] = root_x;
        }
    }

    fn find_root(&mut self, token: usize) -> usize {
        if token != self.root[token] {
            self.root[token] = self.find_root(self.root[token]);
        }
        self.root[token]
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut eq_clusters = UnionFind::new();

        s1.chars().zip(s2.chars()).for_each(|(c1, c2)| {
            eq_clusters.union(ch_to_idx(c1), ch_to_idx(c2));
        });

        base_str
            .chars()
            .map(|ch| (eq_clusters.find_root(ch_to_idx(ch)) as u8 + b'a') as char)
            .collect()
    }
}
