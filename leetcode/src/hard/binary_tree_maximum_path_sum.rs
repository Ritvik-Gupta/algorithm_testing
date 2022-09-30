crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<T>>;

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

struct LinkPath {
    max_link: i32,
    max_path: i32,
}

const MIN_NODE_LINK_PATH: LinkPath = LinkPath {
    max_link: -1001,
    max_path: -1001,
};

fn max_link_and_path(root: Link<TreeNode>) -> LinkPath {
    let root_val = root.borrow().val;

    let (left, right) = (
        root.borrow()
            .left
            .clone()
            .map(max_link_and_path)
            .unwrap_or(MIN_NODE_LINK_PATH),
        root.borrow()
            .right
            .clone()
            .map(max_link_and_path)
            .unwrap_or(MIN_NODE_LINK_PATH),
    );

    let max_link = root_val + max![0, left.max_link, right.max_link];
    LinkPath {
        max_link,
        max_path: max![
            max_link,
            left.max_path,
            right.max_path,
            root_val + left.max_link + right.max_link,
        ],
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Link<TreeNode>>) -> i32 {
        root.map(|root| max_link_and_path(root).max_path)
            .unwrap_or(0)
    }
}
