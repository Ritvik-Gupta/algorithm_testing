pub mod biweekly_contest_80;
pub mod biweekly_contest_81;
pub mod biweekly_contest_89;
pub mod biweekly_contest_90;
pub mod biweekly_contest_91;
pub mod easy;
pub mod hard;
pub mod medium;
pub mod weekly_contest_274;
pub mod weekly_contest_293;
pub mod weekly_contest_297;
pub mod weekly_contest_298;
pub mod weekly_contest_310;
pub mod weekly_contest_312;
pub mod weekly_contest_313;
pub mod weekly_contest_315;
pub mod weekly_contest_317;

macro_rules! solution {
    () => {
        pub struct Solution;
    };
}
pub(crate) use solution;

macro_rules! linked_list_definition {
    () => {
        // Definition for singly-linked list.
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub struct ListNode {
            pub val: i32,
            pub next: Option<Box<ListNode>>,
        }

        impl ListNode {
            #[inline]
            #[allow(dead_code)]
            fn new(val: i32) -> Self {
                ListNode { next: None, val }
            }
        }
    };
}
pub(crate) use linked_list_definition;

macro_rules! binary_tree_definition {
    () => {
        // Definition for a binary tree node.
        #[derive(Debug, PartialEq, Eq)]
        pub struct TreeNode {
            pub val: i32,
            pub left: Option<Rc<RefCell<TreeNode>>>,
            pub right: Option<Rc<RefCell<TreeNode>>>,
        }

        impl TreeNode {
            #[inline]
            #[allow(dead_code)]
            pub fn new(val: i32) -> Self {
                TreeNode {
                    val,
                    left: None,
                    right: None,
                }
            }
        }
    };
}
pub(crate) use binary_tree_definition;
