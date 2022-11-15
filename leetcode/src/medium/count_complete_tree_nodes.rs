crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (left_node, right_node) = match root {
            Some(root) => {
                let root = root.borrow();
                (root.left.clone(), root.right.clone())
            }
            None => return 0,
        };

        let (mut l_ptr, mut r_ptr) = (left_node.clone(), right_node.clone());
        let (mut l_depth, mut r_depth) = (1, 1);

        while let Some(next_node) = l_ptr {
            l_ptr = next_node.borrow().left.clone();
            l_depth += 1;
        }
        while let Some(next_node) = r_ptr {
            r_ptr = next_node.borrow().right.clone();
            r_depth += 1;
        }

        match l_depth == r_depth {
            true => (1 << l_depth) - 1,
            false => 1 + Solution::count_nodes(left_node) + Solution::count_nodes(right_node),
        }
    }
}
