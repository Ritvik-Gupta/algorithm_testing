crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur_max_path(root: &Option<TreeLink>) -> i32 {
    root.as_ref()
        .map(|root| {
            let root = root.borrow();
            1 + i32::max(recur_max_path(&root.left), recur_max_path(&root.right))
        })
        .unwrap_or(0)
}

impl Solution {
    pub fn max_depth(root: Option<TreeLink>) -> i32 {
        recur_max_path(&root)
    }
}
