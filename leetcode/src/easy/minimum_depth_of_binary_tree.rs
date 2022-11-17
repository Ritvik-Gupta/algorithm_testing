crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn min_depth(root: Option<TreeLink>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut node_queue = VecDeque::new();
        node_queue.push_back((1, root.unwrap()));

        while let Some((depth, node)) = node_queue.pop_front() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }

            if let Some(left_node) = node.left.clone() {
                node_queue.push_back((depth + 1, left_node));
            }
            if let Some(right_node) = node.right.clone() {
                node_queue.push_back((depth + 1, right_node));
            }
        }
        unreachable!()
    }
}
