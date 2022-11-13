crate::solution!();
crate::binary_tree_definition!();

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

fn cycle_sort_operations(nums: &mut Vec<i32>) -> i32 {
    let mut visited_table = vec![false; nums.len()];
    let index_table = nums
        .iter()
        .enumerate()
        .map(|(idx, &num)| (num, idx))
        .collect::<HashMap<_, _>>();

    nums.sort();

    (0..nums.len())
        .map(|i| {
            let (mut j, mut cycle_size) = (i, 0);
            while !visited_table[j] {
                visited_table[j] = true;
                j = index_table[&nums[j]];
                cycle_size += 1;
            }
            if cycle_size == 0 {
                0
            } else {
                cycle_size - 1
            }
        })
        .sum()
}

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

            result += cycle_sort_operations(&mut lvl_nodes);

            lvl_nodes.clear();
            num_lvl_nodes = num_nxt_nodes;
            num_nxt_nodes = 0;
        }

        result as i32
    }
}
