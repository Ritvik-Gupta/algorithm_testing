crate::solution!();
crate::binary_tree_definition!();

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Link<T> = Rc<RefCell<T>>;

macro_rules! max {
    { $first_arg: expr } => { $first_arg };
    { $first_arg: expr, $($args: expr),+ $(,)? } => { max($first_arg, max![ $($args),+ ]) };
}

struct UnivalueLinkPath {
    link_for: i32,
    max_link: i32,
    max_path: i32,
}

fn univalue_link_and_path(root: Link<TreeNode>) -> UnivalueLinkPath {
    let root_borrow = root.borrow();
    let root_val = root_borrow.val;

    match (root_borrow.left.clone(), root_borrow.right.clone()) {
        (None, None) => UnivalueLinkPath {
            link_for: root_val,
            max_link: 0,
            max_path: 0,
        },
        (Some(left), Some(right)) => {
            let (left, right) = (univalue_link_and_path(left), univalue_link_and_path(right));
            let max_child_path = max![left.max_path, right.max_path];

            if root_val == left.link_for && root_val == right.link_for {
                UnivalueLinkPath {
                    link_for: root_val,
                    max_link: max![left.max_link, right.max_link] + 1,
                    max_path: max![left.max_link + 1 + right.max_link + 1, max_child_path],
                }
            } else if root_val == left.link_for {
                UnivalueLinkPath {
                    link_for: root_val,
                    max_link: left.max_link + 1,
                    max_path: max![left.max_link + 1, max_child_path],
                }
            } else if root_val == right.link_for {
                UnivalueLinkPath {
                    link_for: root_val,
                    max_link: right.max_link + 1,
                    max_path: max![right.max_link + 1, max_child_path],
                }
            } else {
                UnivalueLinkPath {
                    link_for: root_val,
                    max_link: 0,
                    max_path: max![max_child_path],
                }
            }
        }
        (left, right) => {
            let child = univalue_link_and_path(left.or(right).unwrap());
            let max_link = if root_val == child.link_for {
                child.max_link + 1
            } else {
                0
            };
            UnivalueLinkPath {
                link_for: root_val,
                max_link,
                max_path: max![max_link, child.max_path],
            }
        }
    }
}

impl Solution {
    pub fn longest_univalue_path(root: Option<Link<TreeNode>>) -> i32 {
        root.map(|root| univalue_link_and_path(root).max_path)
            .unwrap_or(0)
    }
}
