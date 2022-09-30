crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn has_palindromic_combination(digit_freq_bitset: u16) -> bool {
    digit_freq_bitset & (digit_freq_bitset - 1) == 0
}

fn palindromic_paths(root: &TreeLink, mut digit_freq_bitset: u16) -> i32 {
    let root = root.borrow();
    digit_freq_bitset ^= 1 << root.val;

    let num_palindromic_paths = if root.left.is_none() && root.right.is_none() {
        if has_palindromic_combination(digit_freq_bitset) {
            1
        } else {
            0
        }
    } else {
        0 + 0
            + root
                .left
                .as_ref()
                .map(|left_node| palindromic_paths(left_node, digit_freq_bitset))
                .unwrap_or(0)
            + root
                .right
                .as_ref()
                .map(|right_node| palindromic_paths(right_node, digit_freq_bitset))
                .unwrap_or(0)
    };

    num_palindromic_paths
}

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<TreeLink>) -> i32 {
        palindromic_paths(&root.unwrap(), 0)
    }
}
