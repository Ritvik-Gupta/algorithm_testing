crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

struct CBTInserter {
    tree: Vec<TreeLink>,
}

impl CBTInserter {
    #[allow(dead_code)]
    fn new(root: Option<TreeLink>) -> Self {
        let root = root.unwrap();

        let mut tree = vec![root];

        let mut i = 0;
        while i < tree.len() {
            let left_node = tree[i].borrow().left.clone();
            if let Some(left_node) = left_node {
                tree.push(left_node);
            }

            let right_node = tree[i].borrow().right.clone();
            if let Some(right_node) = right_node {
                tree.push(right_node);
            }

            i += 1;
        }

        Self { tree }
    }

    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> i32 {
        let n = self.tree.len();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        self.tree.push(node.clone());

        let mut parent_node = self.tree[(n - 1) / 2].borrow_mut();
        match n & 1 {
            0 => parent_node.left = Some(node.clone()),
            _ => parent_node.right = Some(node.clone()),
        }
        parent_node.val
    }

    #[allow(dead_code)]
    fn get_root(&self) -> Option<TreeLink> {
        Some(self.tree[0].clone())
    }
}
