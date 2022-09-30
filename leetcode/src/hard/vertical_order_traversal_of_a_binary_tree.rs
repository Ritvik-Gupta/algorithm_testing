crate::solution!();
crate::binary_tree_definition!();

use std::cell::{Ref, RefCell};
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

struct PositionedNode {
    node: Rc<RefCell<TreeNode>>,
    depth: usize,
    column: isize,
}

impl PositionedNode {
    fn new(node: Rc<RefCell<TreeNode>>, depth: usize, column: isize) -> Self {
        Self {
            node,
            depth,
            column,
        }
    }

    fn attr(&self) -> Ref<TreeNode> {
        self.node.borrow()
    }
}

impl PartialEq for PositionedNode {
    fn eq(&self, other: &Self) -> bool {
        other.depth == self.depth && other.attr().val == self.attr().val
    }
}

impl Eq for PositionedNode {}

use std::cmp::Ordering::{self, *};

impl PartialOrd for PositionedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match other.depth.cmp(&self.depth) {
            Equal => other.attr().val.cmp(&self.attr().val),
            order => order,
        })
    }
}

impl Ord for PositionedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut neg_offset = 0;
        let mut vertical_table = VecDeque::new();

        let mut bfs_queue = VecDeque::new();
        bfs_queue.push_back(PositionedNode::new(root.unwrap(), 0, 0));

        while !bfs_queue.is_empty() {
            for _ in 0..bfs_queue.len() {
                let node = bfs_queue.pop_front().unwrap();

                if let Some(left_node) = &node.attr().left {
                    bfs_queue.push_back(PositionedNode::new(
                        left_node.clone(),
                        node.depth + 1,
                        node.column - 1,
                    ));
                }
                if let Some(right_node) = &node.attr().right {
                    bfs_queue.push_back(PositionedNode::new(
                        right_node.clone(),
                        node.depth + 1,
                        node.column + 1,
                    ));
                }

                if node.column - neg_offset < 0 {
                    vertical_table.push_front(BinaryHeap::new());
                    neg_offset -= 1;
                } else if (node.column - neg_offset) as usize >= vertical_table.len() {
                    vertical_table.push_back(BinaryHeap::new());
                }

                vertical_table[(node.column - neg_offset) as usize].push(node);
            }
        }

        vertical_table
            .iter_mut()
            .map(|level_order_heap| {
                let mut ordered_nodes = Vec::with_capacity(level_order_heap.len());
                while let Some(node) = level_order_heap.pop() {
                    ordered_nodes.push(node.attr().val);
                }
                ordered_nodes
            })
            .collect()
    }
}
