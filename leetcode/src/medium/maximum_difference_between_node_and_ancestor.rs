crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Rc<RefCell<TreeNode>>;

macro_rules! max {
    {$first_arg: expr, $($args: expr),+ $(,)?} => {{
        use std::cmp::max;

        let mut max_val = $first_arg;
        $(
            max_val = max(max_val, $args);
        )+
        max_val
    }};
}

struct AncestorDiff {
    max_encountered: i32,
    min_node: i32,
    max_node: i32,
}

fn recur_max_ancestor_diff(root: &TreeLink) -> AncestorDiff {
    let root = root.borrow();
    let (mut max_encountered, mut min_node, mut max_node) = (0, root.val, root.val);

    if root.left.is_none() && root.right.is_none() {
        return AncestorDiff {
            max_encountered,
            min_node,
            max_node,
        };
    }

    if let Some(left_node) = &root.left {
        let res = recur_max_ancestor_diff(left_node);

        max_encountered = max!(
            max_encountered,
            res.max_encountered,
            (res.max_node - root.val).abs(),
            (res.min_node - root.val).abs()
        );
        min_node = i32::min(min_node, res.min_node);
        max_node = i32::max(max_node, res.max_node);
    }

    if let Some(right_node) = &root.right {
        let res = recur_max_ancestor_diff(right_node);

        max_encountered = max!(
            max_encountered,
            res.max_encountered,
            (res.max_node - root.val).abs(),
            (res.min_node - root.val).abs()
        );
        min_node = i32::min(min_node, res.min_node);
        max_node = i32::max(max_node, res.max_node);
    }

    AncestorDiff {
        max_encountered,
        min_node,
        max_node,
    }
}

impl Solution {
    pub fn max_ancestor_diff(root: Option<TreeLink>) -> i32 {
        recur_max_ancestor_diff(&root.unwrap()).max_encountered
    }
}
