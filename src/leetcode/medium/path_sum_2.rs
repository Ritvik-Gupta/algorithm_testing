crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

fn find_target_paths(
    root: &Rc<RefCell<TreeNode>>,
    mut target_left: i64,
    curr_path: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    let root = root.borrow();

    curr_path.push(root.val);
    target_left -= root.val as i64;
    if root.left.is_none() && root.right.is_none() && target_left == 0 {
        result.push(curr_path.clone());
    }

    root.left
        .as_ref()
        .map(|left_node| find_target_paths(left_node, target_left, curr_path, result));
    root.right
        .as_ref()
        .map(|right_node| find_target_paths(right_node, target_left, curr_path, result));

    curr_path.pop();
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        root.as_ref()
            .map(|root| {
                let mut all_paths = Vec::new();
                find_target_paths(root, target_sum as i64, &mut Vec::new(), &mut all_paths);
                all_paths
            })
            .unwrap_or(Vec::new())
    }
}
