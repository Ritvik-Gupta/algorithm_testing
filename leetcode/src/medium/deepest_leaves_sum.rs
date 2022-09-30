crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::Ordering::{Equal, Greater};

fn dfs_deepest_leaves_sum(
    root: &Option<Rc<RefCell<TreeNode>>>,
    depth: i32,
    tree_depth: &mut i32,
    leaves_sum: &mut i32,
) {
    match root {
        None => {}
        Some(root) => {
            let root = root.borrow();

            dfs_deepest_leaves_sum(&root.left, depth + 1, tree_depth, leaves_sum);
            dfs_deepest_leaves_sum(&root.right, depth + 1, tree_depth, leaves_sum);

            match depth.cmp(tree_depth) {
                Greater => {
                    *tree_depth = depth;
                    *leaves_sum = root.val;
                }
                Equal => *leaves_sum += root.val,
                _ => {}
            }
        }
    }
}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tree_depth = 0;
        let mut leaves_sum = 0;
        dfs_deepest_leaves_sum(&root, 0, &mut tree_depth, &mut leaves_sum);
        leaves_sum
    }
}
