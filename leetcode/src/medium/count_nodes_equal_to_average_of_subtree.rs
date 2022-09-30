crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct AveragePair {
    numerator: i32,
    denominator: i32,
}

impl AveragePair {
    fn from_child_pairs(root_value: i32, left_avg_pair: Self, right_avg_pair: Self) -> Self {
        Self {
            numerator: root_value + left_avg_pair.numerator + right_avg_pair.numerator,
            denominator: 1 + left_avg_pair.denominator + right_avg_pair.denominator,
        }
    }

    fn average(&self) -> i32 {
        self.numerator / self.denominator
    }
}

fn dfs_for_average(
    root: &Option<Rc<RefCell<TreeNode>>>,
    count_having_average: &mut i32,
) -> AveragePair {
    match root {
        Some(root) => {
            let root = root.borrow();

            let avg_pair = AveragePair::from_child_pairs(
                root.val,
                dfs_for_average(&root.left, count_having_average),
                dfs_for_average(&root.right, count_having_average),
            );
            if root.val == avg_pair.average() {
                *count_having_average += 1;
            }

            avg_pair
        }
        None => AveragePair::default(),
    }
}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count_having_average = 0;
        dfs_for_average(&root, &mut count_having_average);
        return count_having_average;
    }
}
