crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

fn recur(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(root) => {
            let root = root.borrow();
            recur(&root.left, result);
            result.push(root.val);
            recur(&root.right, result);
        }
        None => {}
    }
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder = Vec::new();
        recur(&root, &mut inorder);
        inorder
    }
}
