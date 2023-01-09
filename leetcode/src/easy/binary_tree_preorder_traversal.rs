crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn preorder_traversal(root: Option<&TreeLink>, preorder: &mut Vec<i32>) -> Option<()> {
    let root = root?.borrow();
    preorder.push(root.val);
    preorder_traversal(root.left.as_ref(), preorder);
    preorder_traversal(root.right.as_ref(), preorder);
    Some(())
}

impl Solution {
    pub fn preorder_traversal(root: Option<TreeLink>) -> Vec<i32> {
        let mut preorder = Vec::new();
        preorder_traversal(root.as_ref(), &mut preorder);
        preorder
    }
}
