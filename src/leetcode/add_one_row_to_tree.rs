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

fn dfs_through_tree(node: Rc<RefCell<TreeNode>>, depth: i32, val: i32, required_depth: i32) {
    if depth == required_depth {
        let mut node = node.borrow_mut();
        let left_child = node.left.replace(Rc::new(RefCell::new(TreeNode::new(val))));
        let right_child = node
            .right
            .replace(Rc::new(RefCell::new(TreeNode::new(val))));

        node.left.as_ref().unwrap().borrow_mut().left = left_child;
        node.right.as_ref().unwrap().borrow_mut().right = right_child;
    } else {
        if let Some(left_child) = &node.borrow().left {
            dfs_through_tree(left_child.clone(), depth + 1, val, required_depth);
        }
        if let Some(right_child) = &node.borrow().right {
            dfs_through_tree(right_child.clone(), depth + 1, val, required_depth);
        }
    }
}

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            Some(_) if depth == 1 => {
                let mut new_root = TreeNode::new(val);
                new_root.left = root;
                Rc::new(RefCell::new(new_root))
            }
            Some(root) => {
                dfs_through_tree(root.clone(), 1, val, depth - 1);
                root
            }
            None => panic!(),
        })
    }
}
