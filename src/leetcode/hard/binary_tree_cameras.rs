crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

const MAX_VAL: i32 = 10e5 as i32;

struct TreeCoverModes(i32, i32, i32);

fn solve(node: &Option<Rc<RefCell<TreeNode>>>) -> TreeCoverModes {
    match node {
        Some(node) => {
            let left_modes = solve(&node.borrow().left);
            let right_modes = solve(&node.borrow().right);

            let min_submode_a = i32::min(left_modes.1, left_modes.2);
            let min_submode_b = i32::min(right_modes.1, right_modes.2);

            TreeCoverModes(
                left_modes.1 + right_modes.1,
                i32::min(left_modes.2 + min_submode_b, right_modes.2 + min_submode_a),
                1 + i32::min(left_modes.0, min_submode_a) + i32::min(right_modes.0, min_submode_b),
            )
        }
        None => TreeCoverModes(0, 0, MAX_VAL),
    }
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mode_values = solve(&root);
        i32::min(mode_values.1, mode_values.2)
    }
}
