crate::leetcode::binary_tree_definition!();

impl std::string::ToString for TreeNode {
    fn to_string(&self) -> String {
        format!(
            "{}:({},{})",
            self.val,
            self.left
                .as_ref()
                .map(|left| left.borrow().to_string())
                .unwrap_or("".to_string()),
            self.right
                .as_ref()
                .map(|right| right.borrow().to_string())
                .unwrap_or("".to_string())
        )
    }
}

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<T>>;

struct Codec;

impl Codec {
    #[allow(dead_code)]
    fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    fn serialize(&self, root: Option<Link<TreeNode>>) -> String {
        match root {
            Some(root) => root.borrow().to_string(),
            None => "".to_string(),
        }
    }

    #[allow(dead_code)]
    fn match_val(data: &mut impl Iterator<Item = char>) -> i32 {
        let mut val = 0;

        for token in data {
            if token == '(' {
                return val;
            }
        }
        panic!();
    }

    #[allow(dead_code)]
    fn deserialize(&self, data: String) -> Option<Link<TreeNode>> {
        if data == "" {
            return None;
        }

        panic!();
    }
}
