crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

macro_rules! for_link_children {
    ($node: expr; $child_name: ident => $child_expression: expr) => {
        let node = $node.borrow();

        if let Some($child_name) = &node.left {
            $child_expression;
        }

        if let Some($child_name) = &node.right {
            $child_expression;
        }
    };
}

impl Solution {
    pub fn sum_even_grandparent(root: Option<TreeLink>) -> i32 {
        let mut sum = 0;
        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.unwrap());

        while let Some(node) = node_queue.pop_front() {
            for_link_children! { node;
                child => {
                    node_queue.push_back(child.clone());
                    if node.borrow().val % 2 == 0 {
                        for_link_children! { child; grandchild => sum += grandchild.borrow().val }
                    }
                }
            }
        }
        sum
    }
}
