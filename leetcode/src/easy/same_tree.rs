crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur_are_similar(p: Option<&TreeLink>, q: Option<&TreeLink>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let (p, q) = (p.borrow(), q.borrow());
            p.val == q.val
                && recur_are_similar(p.left.as_ref(), q.left.as_ref())
                && recur_are_similar(p.right.as_ref(), q.right.as_ref())
        }
        _ => p.is_none() && q.is_none(),
    }
}

impl Solution {
    pub fn is_same_tree(p: Option<TreeLink>, q: Option<TreeLink>) -> bool {
        recur_are_similar(p.as_ref(), q.as_ref())
    }
}
