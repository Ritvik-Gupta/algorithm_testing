crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

struct TreeCollector(Vec<Vec<i32>>);

impl TreeCollector {
    fn for_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut tree_collector = Self(Vec::new());
        if let Some(root) = root {
            tree_collector.collect(&root, 0);
        }
        tree_collector
            .0
            .iter_mut()
            .enumerate()
            .for_each(|(pos, store)| {
                if pos % 2 == 1 {
                    *store = store.iter().map(|&x| x).rev().collect();
                }
            });
        tree_collector.0
    }

    fn collect(&mut self, node: &Rc<RefCell<TreeNode>>, depth: usize) {
        if self.0.len() == depth {
            self.0.push(Vec::new());
        }
        let node = node.borrow();
        self.0[depth].push(node.val);
        node.left
            .as_ref()
            .map(|left_child| self.collect(left_child, depth + 1));
        node.right
            .as_ref()
            .map(|right_child| self.collect(right_child, depth + 1));
    }
}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        TreeCollector::for_tree(root)
    }
}
