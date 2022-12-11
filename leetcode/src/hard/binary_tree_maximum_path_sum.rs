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

struct LinkPath {
    max_link: i32,
    max_path: i32,
}

const MIN_NODE_LINK_PATH: LinkPath = LinkPath {
    max_link: -1001,
    max_path: -1001,
};

fn max_link_and_path(root: &Option<TreeLink>) -> LinkPath {
    let root = match root {
        Some(root) => root,
        None => return MIN_NODE_LINK_PATH,
    };
    let root = root.borrow();

    let (left, right) = (
        max_link_and_path(&root.left),
        max_link_and_path(&root.right),
    );

    let max_link = root.val + max![0, left.max_link, right.max_link];
    LinkPath {
        max_link,
        max_path: max![
            max_link,
            left.max_path,
            right.max_path,
            root.val + left.max_link + right.max_link,
        ],
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<TreeLink>) -> i32 {
        if root.is_none() {
            return 0;
        }
        max_link_and_path(&root).max_path
    }
}
