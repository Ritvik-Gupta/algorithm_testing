use crate::trie::{
    aho_corasick::AhoCorasickTrie,
    TrieNode,
    TrieNodeType::{Token, ROOT},
};
use std::{cell::RefCell, rc::Rc};

pub struct TrieLink(Rc<RefCell<TrieNode>>);

impl TrieLink {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(TrieNode::default())))
    }
    pub fn relative_parent_with_token(&self) -> (char, Self) {
        match &self.borrow().of_type {
            ROOT => (crate::trie::DEFAULT_TRIE_TOKEN, self.clone()),
            Token { parent, token } => (*token, parent.clone()),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = self.clone();
        for token in word.chars() {
            current_node = {
                let mut current_node_borrow = current_node.borrow_mut();
                let current_node_depth = current_node_borrow.depth;
                current_node_borrow
                    .children
                    .entry(token)
                    .or_insert_with(|| {
                        Self(Rc::new(RefCell::new(TrieNode {
                            depth: current_node_depth + 1,
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

impl std::hash::Hash for TrieLink {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.borrow().hash(state);
    }
}

impl std::cmp::PartialEq for TrieLink {
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl std::cmp::Eq for TrieLink {}

impl Clone for TrieLink {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
