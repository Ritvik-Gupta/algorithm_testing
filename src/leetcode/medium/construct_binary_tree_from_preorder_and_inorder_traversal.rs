crate::leetcode::solution!();
crate::leetcode::binary_tree_definition!();

use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Solution {
    fn build_node(
        preorder: &[i32],
        inorder: &[i32],
        inorder_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let mut node = TreeNode::new(preorder[0]);
        let in_root_idx = inorder_map[&node.val];
        let partition_idx = in_root_idx - inorder_map[&inorder[0]];

        node.left = Solution::build_node(
            &preorder[1..partition_idx + 1],
            &inorder[..partition_idx],
            inorder_map,
        );
        node.right = Solution::build_node(
            &preorder[partition_idx + 1..],
            &inorder[partition_idx + 1..],
            inorder_map,
        );

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_node(
            &preorder,
            &inorder,
            &inorder
                .iter()
                .enumerate()
                .map(|(idx, &val)| (val, idx))
                .collect(),
        )
    }
}
