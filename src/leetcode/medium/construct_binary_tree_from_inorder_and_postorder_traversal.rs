crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Solution {
    fn build_node(
        inorder: &[i32],
        postorder: &[i32],
        inorder_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }

        let mut node = TreeNode::new(*postorder.last().unwrap());
        let in_root_idx = inorder_map[&node.val];
        let partition_idx = in_root_idx - inorder_map[&inorder[0]];

        node.left = Solution::build_node(
            &inorder[..partition_idx],
            &postorder[..partition_idx],
            inorder_map,
        );
        node.right = Solution::build_node(
            &inorder[partition_idx + 1..],
            &postorder[partition_idx..postorder.len() - 1],
            inorder_map,
        );

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_node(
            &inorder,
            &postorder,
            &inorder
                .iter()
                .enumerate()
                .map(|(idx, &val)| (val, idx))
                .collect(),
        )
    }
}
