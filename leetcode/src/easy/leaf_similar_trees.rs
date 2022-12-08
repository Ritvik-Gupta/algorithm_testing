crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

struct TreeIterator {
    callstack: Vec<(TreeLink, bool)>,
}

impl TreeIterator {
    fn new(root: TreeLink) -> Self {
        let redirected_root = Rc::new(RefCell::new(TreeNode::new(0)));
        redirected_root.borrow_mut().left = Some(root);

        Self {
            callstack: vec![(redirected_root, false)],
        }
    }
}

impl Iterator for TreeIterator {
    type Item = TreeLink;

    fn next(&mut self) -> Option<Self::Item> {
        match self.callstack.last_mut() {
            Some((node, second_visit)) => {
                if *second_visit {
                    let right_node = node.borrow().right.clone();
                    self.callstack.pop();
                    match right_node {
                        Some(right_node) => {
                            self.callstack.push((Rc::clone(&right_node), false));
                            Some(right_node)
                        }
                        None => self.next(),
                    }
                } else {
                    let left_node = node.borrow().left.clone();
                    *second_visit = true;
                    match left_node {
                        Some(left_node) => {
                            self.callstack.push((Rc::clone(&left_node), false));
                            Some(left_node)
                        }
                        None => self.next(),
                    }
                }
            }
            None => None,
        }
    }
}

fn is_leaf(root: &TreeLink) -> bool {
    root.borrow().left.is_none() && root.borrow().right.is_none()
}

impl Solution {
    pub fn leaf_similar(root1: Option<TreeLink>, root2: Option<TreeLink>) -> bool {
        let leaf_itr1 = TreeIterator::new(root1.unwrap())
            .filter(is_leaf)
            .map(|node| node.borrow().val);

        let leaf_itr2 = TreeIterator::new(root2.unwrap())
            .filter(is_leaf)
            .map(|node| node.borrow().val);

        leaf_itr1.eq(leaf_itr2)
    }
}
