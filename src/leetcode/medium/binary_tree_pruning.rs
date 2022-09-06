crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

fn tree_withstands_prune(root: &TreeLink) -> bool {
    match root {
        Some(root) => {
            let root_borrow = root.borrow();
            let (left_stands, right_stands) = (
                tree_withstands_prune(&root_borrow.left),
                tree_withstands_prune(&root_borrow.right),
            );
            let is_unit_node = root_borrow.val == 1;
            let subtree_withstands = left_stands || right_stands || is_unit_node;

            if !subtree_withstands {
                return false;
            }
            drop(root_borrow);

            if !left_stands {
                root.borrow_mut().left = None;
            }
            if !right_stands {
                root.borrow_mut().right = None;
            }

            subtree_withstands
        }
        None => false,
    }
}

impl Solution {
    pub fn prune_tree(root: TreeLink) -> TreeLink {
        match tree_withstands_prune(&root) {
            true => root,
            false => None,
        }
    }
}
