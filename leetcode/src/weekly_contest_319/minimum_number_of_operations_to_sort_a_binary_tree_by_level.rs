crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn minimum_operations(root: Option<TreeLink>) -> i32 {
        let mut result = 0;

        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.unwrap());
        let (mut num_lvl_nodes, mut num_nxt_nodes) = (1, 0);
        let mut lvl_nodes = Vec::new();

        while !node_queue.is_empty() {
            for _ in 0..num_lvl_nodes {
                let node = node_queue.pop_front().unwrap();
                let node = node.borrow();

                lvl_nodes.push(node.val);

                if let Some(left_node) = &node.left {
                    node_queue.push_back(Rc::clone(left_node));
                    num_nxt_nodes += 1;
                }

                if let Some(right_node) = &node.right {
                    node_queue.push_back(Rc::clone(right_node));
                    num_nxt_nodes += 1;
                }
            }

            let mut sorted_nodes = lvl_nodes.clone();
            sorted_nodes.sort();

            let dicontinuities = lvl_nodes
                .iter()
                .zip(sorted_nodes.iter())
                .filter(|(x, y)| x != y)
                .count();
            result += (1 + dicontinuities) / 2;

            lvl_nodes.clear();
            num_lvl_nodes = num_nxt_nodes;
            num_nxt_nodes = 0;
        }

        result as i32
    }
}
