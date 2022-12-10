crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur_total_sum(root: &Option<TreeLink>) -> u128 {
    let root = match root {
        Some(root) => root,
        None => return 0,
    };
    let root = root.borrow();

    root.val as u128 + recur_total_sum(&root.left) + recur_total_sum(&root.right)
}

fn traverse_find_max_prod(root: &Option<TreeLink>, root_sum: u128, max_prod: &mut u128) -> u128 {
    let root = match root {
        Some(root) => root,
        None => return 0,
    };
    let root = root.borrow();

    let left = traverse_find_max_prod(&root.left, root_sum, max_prod);
    let right = traverse_find_max_prod(&root.right, root_sum, max_prod);

    *max_prod = u128::max(*max_prod, (root_sum - left) * left);
    *max_prod = u128::max(*max_prod, (root_sum - right) * right);

    root.val as u128 + left + right
}

impl Solution {
    pub fn max_product(root: Option<TreeLink>) -> i32 {
        let mut max_prod = 0;
        traverse_find_max_prod(&root, recur_total_sum(&root), &mut max_prod);
        (max_prod % 1000000007) as i32
    }
}
