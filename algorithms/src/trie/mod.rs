pub mod aho_corasick;
pub mod display;
pub mod link;

use link::TrieLink;
use std::collections::BTreeMap;

static mut TOTAL_TRIE_NODES: u16 = 0;
const DEFAULT_TRIE_TOKEN: char = '_';

#[derive(Debug)]
pub struct TrieNode {
    index: u16,
    depth: usize,
    is_end: bool,
    of_type: TrieNodeType,
    children: BTreeMap<char, TrieLink>,
    suffix: Option<TrieSuffix>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum TrieNodeType {
    ROOT,
    Token { token: char, parent: TrieLink },
}

#[derive(Clone, Debug)]
struct TrieSuffix {
    failure: TrieLink,
    dictionary: Option<TrieLink>,
}

impl TrieNode {
    fn token_or_default(&self) -> char {
        use TrieNodeType::{Token, ROOT};

        match &self.of_type {
            ROOT => DEFAULT_TRIE_TOKEN,
            Token { token, .. } => *token,
        }
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        unsafe { TOTAL_TRIE_NODES += 1 };
        Self {
            index: unsafe { TOTAL_TRIE_NODES },
            depth: 0,
            is_end: false,
            of_type: TrieNodeType::ROOT,
            children: BTreeMap::new(),
            suffix: None,
        }
    }
}

impl std::hash::Hash for TrieNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_end.hash(state);
        self.of_type.hash(state);
    }
}

impl std::cmp::PartialEq for TrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.is_end == other.is_end && self.of_type == other.of_type
    }
}

impl std::cmp::Eq for TrieNode {}
