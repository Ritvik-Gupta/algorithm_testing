#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Link<TreeNode>>,
    pub right: Option<Link<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<T>>;

struct PathSumStore {
    target_sum: i32,
    total_count: i32,
    parent_path_sums: Vec<i32>,
}

impl PathSumStore {
    fn new(target_sum: i32) -> Self {
        PathSumStore {
            target_sum,
            total_count: 0,
            parent_path_sums: Vec::new(),
        }
    }

    fn find_path_sum(&mut self, root: Link<TreeNode>) {
        let root_borrow = root.borrow();

        for path_sum in self.parent_path_sums.iter_mut() {
            *path_sum = *path_sum + root_borrow.val;
            if *path_sum == self.target_sum {
                self.total_count += 1;
            }
        }

        self.parent_path_sums.push(root_borrow.val);
        if root_borrow.val == self.target_sum {
            self.total_count += 1;
        }

        root_borrow
            .left
            .clone()
            .map(|left| self.find_path_sum(left));

        root_borrow
            .right
            .clone()
            .map(|right| self.find_path_sum(right));

        self.parent_path_sums.pop();

        self.parent_path_sums
            .iter_mut()
            .for_each(|path_sum| *path_sum = *path_sum - root_borrow.val);
    }
}

impl Solution {
    pub fn path_sum(root: Option<Link<TreeNode>>, target_sum: i32) -> i32 {
        root.map(|root| {
            let mut path_sum_store = PathSumStore::new(target_sum);
            path_sum_store.find_path_sum(root);
            path_sum_store.total_count
        })
        .unwrap_or(0)
    }
}
