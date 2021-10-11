use std::{cell::RefCell, rc::Rc};

pub struct TrieLink(Rc<RefCell<TrieNode>>);

impl TrieLink {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(TrieNode::default())))
    }
}

impl Clone for TrieLink {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

use crate::trie::TrieNode;

impl std::ops::Deref for TrieLink {
    type Target = Rc<RefCell<TrieNode>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TrieLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use crate::trie::{aho_corasick::AhoCorasickTrie, TrieNodeType::Token};

impl TrieLink {
    pub fn insert(&mut self, word: &str) {
        let mut current_node = self.clone();
        for token in word.chars() {
            current_node = {
                let mut current_node_borrow = current_node.borrow_mut();
                current_node_borrow
                    .children
                    .entry(token)
                    .or_insert_with(|| {
                        Self(Rc::new(RefCell::new(TrieNode {
                            of_type: Token {
                                token,
                                parent: current_node.clone(),
                            },
                            ..Default::default()
                        })))
                    })
                    .clone()
            };
        }
        current_node.borrow_mut().is_end = true;
    }

    pub fn lock(&mut self) -> AhoCorasickTrie {
        AhoCorasickTrie::new(self)
    }
}
