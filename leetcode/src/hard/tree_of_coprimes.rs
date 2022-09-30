crate::solution!();

use std::collections::HashMap;

const MAX_NODE_VAL_STATE: usize = 51;

struct GcdCalculator(Vec<Vec<Option<usize>>>);

impl GcdCalculator {
    fn new() -> Self {
        Self(vec![vec![None; MAX_NODE_VAL_STATE]; MAX_NODE_VAL_STATE])
    }

    fn gcd(&mut self, x: usize, y: usize) -> usize {
        match (x, &self.0[x][y], &self.0[y][x]) {
            (0, _, _) => y,
            (_, Some(val), _) | (_, _, Some(val)) => *val,
            _ => {
                let value = self.gcd(y % x, x);
                self.0[x][y] = Some(value);
                self.0[y][x] = Some(value);
                value
            }
        }
    }
}

struct Tree {
    tree_nodes_record: HashMap<i32, Vec<i32>>,
    values: Vec<i32>,
}

impl Tree {
    fn new(edges: Vec<Vec<i32>>, values: Vec<i32>) -> Self {
        let mut tree_nodes_record = HashMap::with_capacity(values.len());
        for edge in edges {
            tree_nodes_record
                .entry(edge[0] as i32)
                .or_insert_with(|| Vec::with_capacity(3))
                .push(edge[1] as i32);

            tree_nodes_record
                .entry(edge[1] as i32)
                .or_insert_with(|| Vec::with_capacity(3))
                .push(edge[0] as i32);
        }

        Self {
            tree_nodes_record,
            values,
        }
    }
}

struct TreeTraverser {
    val_states: Vec<(i32, i32)>,
    calculator: GcdCalculator,
}

impl TreeTraverser {
    fn new() -> Self {
        Self {
            val_states: vec![(-1, -1); MAX_NODE_VAL_STATE],
            calculator: GcdCalculator::new(),
        }
    }

    fn build_coprimes(
        &mut self,
        node_id: i32,
        parent_node_id: i32,
        depth: i32,
        tree: &Tree,
        result: &mut Vec<i32>,
    ) {
        let node_val = tree.values[node_id as usize];

        let mut latest_id = 0;
        for i in 0..MAX_NODE_VAL_STATE {
            if self.calculator.gcd(node_val as usize, i) == 1
                && self.val_states[latest_id].0 < self.val_states[i].0
            {
                latest_id = i;
            }
        }

        result[node_id as usize] = self.val_states[latest_id].1;

        if let Some(child_nodes) = tree.tree_nodes_record.get(&node_id) {
            let prev_val_state = self.val_states[node_val as usize];
            self.val_states[node_val as usize] = (depth, node_id);

            child_nodes
                .iter()
                .filter(|&&child_node_id| child_node_id != parent_node_id)
                .for_each(|&child_node_id| {
                    self.build_coprimes(child_node_id, node_id, depth + 1, tree, result)
                });

            self.val_states[node_val as usize] = prev_val_state;
        }
    }
}

impl Solution {
    pub fn get_coprimes(values: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![-1; values.len()];
        TreeTraverser::new().build_coprimes(0, -1, 0, &Tree::new(edges, values), &mut result);
        result
    }
}
