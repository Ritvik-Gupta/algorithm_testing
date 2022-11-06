crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

fn recur_add_path(root: &TreeLink, mut prev_path: String, paths: &mut Vec<String>) {
    let root = root.borrow();

    prev_path.push_str(&root.val.to_string());
    if root.left.is_none() && root.right.is_none() {
        paths.push(prev_path.clone());
        return;
    }
    prev_path.push_str("->");

    if let Some(left_node) = &root.left {
        recur_add_path(left_node, prev_path.clone(), paths);
    }
    if let Some(right_node) = &root.right {
        recur_add_path(right_node, prev_path.clone(), paths);
    }
}

impl Solution {
    pub fn binary_tree_paths(root: Option<TreeLink>) -> Vec<String> {
        let mut paths = Vec::new();
        recur_add_path(&root.unwrap(), String::new(), &mut paths);
        paths
    }
}
