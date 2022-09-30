crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type LinkNode = Rc<RefCell<TreeNode>>;

fn find_path_sum(
    root: &Option<LinkNode>,
    target_sum: i32,
    parent_path_sums: &mut Vec<i128>,
) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root = root.as_ref().unwrap();

    let mut total_count = 0;

    let root_borrow = root.borrow();
    if root_borrow.val == target_sum {
        total_count += 1;
    }

    parent_path_sums.iter_mut().for_each(|path_sum| {
        *path_sum = *path_sum + root_borrow.val as i128;
        if *path_sum == target_sum as i128 {
            total_count += 1;
        }
    });
    parent_path_sums.push(root_borrow.val as i128);

    total_count += find_path_sum(&root_borrow.left, target_sum, parent_path_sums);
    total_count += find_path_sum(&root_borrow.right, target_sum, parent_path_sums);

    parent_path_sums.pop();
    parent_path_sums
        .iter_mut()
        .for_each(|path_sum| *path_sum = *path_sum - root_borrow.val as i128);

    total_count
}

impl Solution {
    pub fn path_sum(root: Option<LinkNode>, target_sum: i32) -> i32 {
        find_path_sum(&root, target_sum, &mut Vec::new())
    }
}
