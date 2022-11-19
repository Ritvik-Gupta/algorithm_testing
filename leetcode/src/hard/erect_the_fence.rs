crate::solution!();

use std::collections::HashSet;

fn loc_tuple(loc: &Vec<i32>) -> (i32, i32) {
    (loc[0], loc[1])
}

fn cross(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
    let ((x1, y1), (x2, y2), (x3, y3)) = (loc_tuple(a), loc_tuple(b), loc_tuple(c));
    (y3 - y2) * (x2 - x1) - (y2 - y1) * (x3 - x2)
}

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        trees.sort();

        let (mut upper, mut lower) = (Vec::new(), Vec::new());
        for tree in trees {
            while upper.len() >= 2
                && cross(&tree, &upper[upper.len() - 1], &upper[upper.len() - 2]) < 0
            {
                upper.pop();
            }
            upper.push(tree.clone());

            while lower.len() >= 2
                && cross(&tree, &lower[lower.len() - 1], &lower[lower.len() - 2]) > 0
            {
                lower.pop();
            }
            lower.push(tree);
        }

        upper
            .into_iter()
            .chain(lower.into_iter())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}
