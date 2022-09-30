crate::solution!();

fn letter_transpose(letter: char) -> usize {
    (letter as u8 - b'a') as usize
}

struct EqnUnionFind {
    root: Vec<usize>,
}

impl EqnUnionFind {
    fn new() -> Self {
        Self {
            root: (0..26).collect(),
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_root(x);
        let root_y = self.find_root(y);

        if root_x == root_y {
            return;
        }
        self.root[root_y] = root_x;
    }

    fn find_root(&mut self, token: usize) -> usize {
        if token != self.root[token] {
            self.root[token] = self.find_root(self.root[token]);
        }
        self.root[token]
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut eqn_clusters = EqnUnionFind::new();
        let mut disjoint_tokens = Vec::new();

        for eqn in equations.iter() {
            let mut eqn = eqn.chars();

            let token_x = letter_transpose(eqn.next().unwrap());
            let is_equal_symbol = eqn.next().unwrap() == '=';
            eqn.next().unwrap();
            let token_y = letter_transpose(eqn.next().unwrap());

            if is_equal_symbol {
                eqn_clusters.union(token_x, token_y);
            } else if eqn_clusters.find_root(token_x) == eqn_clusters.find_root(token_y) {
                return false;
            } else {
                disjoint_tokens.push((token_x, token_y));
            }
        }

        disjoint_tokens.iter().all(|&(token_x, token_y)| {
            eqn_clusters.find_root(token_x) != eqn_clusters.find_root(token_y)
        })
    }
}
