pub mod aho_corasick;
pub mod display;
pub mod link;

use link::TrieLink;
use std::collections::BTreeMap;

type TrieNodeIndex = u16;
static mut TOTAL_TRIE_NODES: TrieNodeIndex = 0;

#[derive(Debug)]
pub struct TrieNode {
    index: TrieNodeIndex,
    is_end: bool,
    of_type: TrieNodeType,
    children: BTreeMap<char, TrieLink>,
    suffix: Option<TrieSuffix>,
}

#[derive(Debug)]
pub enum TrieNodeType {
    ROOT,
    Token { token: char, parent: TrieLink },
}

#[derive(Clone, Debug)]
struct TrieSuffix {
    failure: TrieLink,
    dictionary: Option<TrieLink>,
}

impl Default for TrieNode {
    fn default() -> Self {
        unsafe { TOTAL_TRIE_NODES += 1 };
        Self {
            index: unsafe { TOTAL_TRIE_NODES },
            is_end: false,
            of_type: TrieNodeType::ROOT,
            children: BTreeMap::new(),
            suffix: None,
        }
    }
}
