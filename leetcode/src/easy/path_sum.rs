crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur(root: &TreeLink, mut target_sum_left: i32) -> bool {
    let root = root.borrow();
    target_sum_left -= root.val;
    if root.left.is_none() && root.right.is_none() {
        return target_sum_left == 0;
    }

    false
        || root
            .left
            .as_ref()
            .map(|left_node| recur(left_node, target_sum_left))
            .unwrap_or(false)
        || root
            .right
            .as_ref()
            .map(|right_node| recur(right_node, target_sum_left))
            .unwrap_or(false)
}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        recur(&root.unwrap(), target_sum)
    }
}
