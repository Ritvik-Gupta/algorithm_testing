crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

fn recur_range_sum(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let root = if let Some(root) = root {
        root.borrow()
    } else {
        return 0;
    };

    if root.val < low {
        recur_range_sum(&root.right, low, high)
    } else if root.val > high {
        recur_range_sum(&root.left, low, high)
    } else {
        root.val + recur_range_sum(&root.left, low, high) + recur_range_sum(&root.right, low, high)
    }
}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        recur_range_sum(&root, low, high)
    }
}
