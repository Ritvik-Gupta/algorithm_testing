crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn traverse_find_root_to_leaf_numbers(root: &TreeLink, mut num_buffer: i32, total_sum: &mut i32) {
    let root = root.borrow();
    num_buffer = num_buffer * 10 + root.val;

    if root.left.is_none() && root.right.is_none() {
        *total_sum += num_buffer;
        return;
    }

    if let Some(left_node) = &root.left {
        traverse_find_root_to_leaf_numbers(left_node, num_buffer, total_sum);
    }
    if let Some(right_node) = &root.right {
        traverse_find_root_to_leaf_numbers(right_node, num_buffer, total_sum);
    }
}

impl Solution {
    pub fn sum_numbers(root: Option<TreeLink>) -> i32 {
        let mut total_sum = 0;
        traverse_find_root_to_leaf_numbers(&root.unwrap(), 0, &mut total_sum);
        total_sum
    }
}
