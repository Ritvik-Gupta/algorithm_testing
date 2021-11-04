pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let root = match root {
            Some(root) => root,
            None => return result,
        };

        let mut depth_nodes = Vec::new();
        depth_nodes.push(root);
        while !depth_nodes.is_empty() {
            let mut next_depth_nodes = Vec::new();
            result.push(
                depth_nodes
                    .into_iter()
                    .inspect(|node| {
                        if let Some(left_node) = node.borrow().left.clone() {
                            next_depth_nodes.push(left_node);
                        }
                        if let Some(right_node) = node.borrow().right.clone() {
                            next_depth_nodes.push(right_node);
                        }
                    })
                    .map(|node| node.borrow().val)
                    .max()
                    .unwrap(),
            );
            depth_nodes = next_depth_nodes;
        }

        result
    }
}
