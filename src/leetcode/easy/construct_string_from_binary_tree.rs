crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

fn recur(root: &Rc<RefCell<TreeNode>>) -> String {
    let root = root.borrow();

    format!(
        "{}{}",
        root.val,
        match (&root.left, &root.right) {
            (Some(left_node), Some(right_node)) =>
                format!("({})({})", recur(left_node), recur(right_node)),
            (None, Some(right_node)) => format!("()({})", recur(right_node)),
            (Some(left_node), None) => format!("({})", recur(left_node),),
            _ => "".to_string(),
        }
    )
}

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        recur(&root.unwrap())
    }
}
