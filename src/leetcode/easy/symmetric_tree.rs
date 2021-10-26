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

fn is_mirror(
    tree_a: Option<&Rc<RefCell<TreeNode>>>,
    tree_b: Option<&Rc<RefCell<TreeNode>>>,
) -> bool {
    match (tree_a, tree_b) {
        (Some(tree_a), Some(tree_b)) => {
            let (tree_a, tree_b) = (tree_a.borrow(), tree_b.borrow());
            tree_a.val == tree_b.val
                && is_mirror(tree_a.left.as_ref(), tree_b.right.as_ref())
                && is_mirror(tree_a.right.as_ref(), tree_b.left.as_ref())
        }
        (None, None) => true,
        _ => false,
    }
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => {
                let root = root.borrow();
                is_mirror(root.left.as_ref(), root.right.as_ref())
            }
            None => panic!(),
        }
    }
}
