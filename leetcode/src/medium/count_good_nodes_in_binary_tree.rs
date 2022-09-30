crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

fn find_good_node_recur(root: &Rc<RefCell<TreeNode>>, mut max_val_encountered: i32) -> i32 {
    let root_ref = root.borrow();
    let mut good_node_weight = 0;

    if root_ref.val >= max_val_encountered {
        max_val_encountered = root_ref.val;
        good_node_weight = 1;
    }

    good_node_weight
        + root_ref
            .left
            .as_ref()
            .map(|left_node| find_good_node_recur(left_node, max_val_encountered))
            .unwrap_or(0)
        + root_ref
            .right
            .as_ref()
            .map(|right_node| find_good_node_recur(right_node, max_val_encountered))
            .unwrap_or(0)
}

const MIN_NODE_VAL: i32 = -10001;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        find_good_node_recur(&root.unwrap(), MIN_NODE_VAL)
    }
}
