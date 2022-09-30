crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

struct Average {
    sum: i128,
    total: usize,
}

impl Average {
    fn zero() -> Self {
        Self { sum: 0, total: 0 }
    }

    fn compute(&self) -> f64 {
        self.sum as f64 / self.total as f64
    }
}

impl std::ops::AddAssign<i32> for Average {
    fn add_assign(&mut self, val: i32) {
        self.sum += val as i128;
        self.total += 1;
    }
}

fn recur(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, level_avgs: &mut Vec<Average>) {
    match root {
        Some(root) => {
            if depth == level_avgs.len() {
                level_avgs.push(Average::zero());
            }

            let root_ref = root.borrow();
            level_avgs[depth] += root_ref.val;

            recur(&root_ref.left, depth + 1, level_avgs);
            recur(&root_ref.right, depth + 1, level_avgs);
        }
        _ => {}
    }
}

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut level_avgs = Vec::new();
        recur(&root, 0, &mut level_avgs);
        level_avgs.iter().map(Average::compute).collect()
    }
}
