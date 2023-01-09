crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn postorder_traversal(root: Option<&TreeLink>, postorder: &mut Vec<i32>) -> Option<()> {
    let root = root?.borrow();
    postorder_traversal(root.left.as_ref(), postorder);
    postorder_traversal(root.right.as_ref(), postorder);
    postorder.push(root.val);
    Some(())
}

impl Solution {
    pub fn postorder_traversal(root: Option<TreeLink>) -> Vec<i32> {
        let mut postorder = Vec::new();
        postorder_traversal(root.as_ref(), &mut postorder);
        postorder
    }
}
