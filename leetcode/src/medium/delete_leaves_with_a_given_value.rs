crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur_remove_leaf_nodes(root: TreeLink, target: i32) -> Option<TreeLink> {
    {
        let mut node = root.borrow_mut();

        if let Some(left_node) = node.left.take() {
            node.left = recur_remove_leaf_nodes(left_node, target);
        }
        if let Some(right_node) = node.right.take() {
            node.right = recur_remove_leaf_nodes(right_node, target);
        }

        if node.left.is_none() && node.right.is_none() && node.val == target {
            return None;
        }
    }

    Some(root)
}

impl Solution {
    pub fn remove_leaf_nodes(root: Option<TreeLink>, target: i32) -> Option<TreeLink> {
        recur_remove_leaf_nodes(root.unwrap(), target)
    }
}
