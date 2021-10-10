pub struct Solution;

use std::collections::BTreeMap;

#[derive(Debug)]
struct DiretoryNode {
    parent_token: String,
    parent: DirectoryLink,
    children_directories: BTreeMap<String, DiretoryNode>,
}

type DirectoryLink = *mut DiretoryNode;

use std::ptr;

const SLASH: u8 = b'/';
static SAME_DIR: &str = ".";
static PARENT_DIR: &str = "..";

impl DiretoryNode {
    fn new_root() -> Self {
        let mut root = Self {
            parent_token: "".to_string(),
            parent: ptr::null_mut(),
            children_directories: BTreeMap::new(),
        };
        root.parent = &mut root;
        root
    }

    fn find_or_create(&mut self, token: String) -> *mut Self {
        if token == SAME_DIR {
            return self;
        } else if token == PARENT_DIR {
            return self.parent;
        }

        let parent = self as *mut _;
        self.children_directories
            .entry(token.clone())
            .or_insert_with(|| Self {
                parent,
                parent_token: token,
                children_directories: BTreeMap::new(),
            })
    }
}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut root_dir = DiretoryNode::new_root();
        let path = path.as_bytes();
        let mut current_dir: *mut _ = &mut root_dir;

        let mut pos = 0;
        let mut collected_dir_name = String::new();
        while pos < path.len() || !collected_dir_name.is_empty() {
            if pos < path.len() && path[pos] != SLASH {
                collected_dir_name.push(path[pos] as char);
            } else if !collected_dir_name.is_empty() {
                current_dir = unsafe { (*current_dir).find_or_create(collected_dir_name) };
                collected_dir_name = String::new();
            }
            pos += 1;
        }

        let mut simplified_name = String::new();
        unsafe {
            loop {
                simplified_name = format!("/{}{}", (*current_dir).parent_token, simplified_name);
                current_dir = (*current_dir).parent;
                if current_dir == (*current_dir).parent {
                    break;
                }
            }
        }
        simplified_name
    }
}
