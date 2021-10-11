use crate::trie::{
    TrieLink, TrieNode, TrieNodeIndex,
    TrieNodeType::{Token, ROOT},
    TrieSuffix,
};
use std::collections::{BTreeMap, VecDeque};

pub struct AhoCorasickTrie<'a> {
    trie: &'a mut TrieLink,
    suffix_transitions: BTreeMap<TrieNodeIndex, BTreeMap<char, TrieLink>>,
}

impl<'a> AhoCorasickTrie<'a> {
    pub fn new(trie: &'a mut TrieLink) -> Self {
        trie.print();
        let mut ac_trie = Self {
            trie,
            suffix_transitions: BTreeMap::new(),
        };
        let mut node_queue = VecDeque::new();
        node_queue.push_back(ac_trie.trie.clone());

        while let Some(current_node) = node_queue.pop_front() {
            current_node
                .borrow()
                .children
                .iter()
                .for_each(|(_, child_node)| node_queue.push_back(child_node.clone()));
            current_node.borrow_mut().suffix = Some(ac_trie.get_suffix(&current_node));
        }
        ac_trie.suffix_transitions.clear();

        println!("\n\n\n");
        ac_trie.trie.print();
        ac_trie
    }

    fn get_suffix(&mut self, node: &TrieLink) -> TrieSuffix {
        match &node.borrow().suffix {
            Some(suffix) => suffix.clone(),
            None => {
                let failure_suffix = match &node.borrow().of_type {
                    ROOT => node.clone(),
                    Token {
                        parent: parent_node,
                        token,
                    } => match parent_node.borrow().of_type {
                        ROOT => parent_node.clone(),
                        _ => {
                            let parent_failure_suffix = self.get_suffix(parent_node).failure;
                            self.get_transition(&parent_failure_suffix, *token)
                        }
                    },
                };

                let dictionary_suffix = match *failure_suffix.borrow() {
                    TrieNode { is_end: true, .. } => Some(failure_suffix.clone()),
                    TrieNode { of_type: ROOT, .. } => None,
                    _ => self.get_suffix(&failure_suffix).dictionary,
                };

                TrieSuffix {
                    failure: failure_suffix,
                    dictionary: dictionary_suffix,
                }
            }
        }
    }

    fn get_transition(&mut self, node: &TrieLink, token: char) -> TrieLink {
        match self
            .suffix_transitions
            .entry(node.borrow().index)
            .or_insert(BTreeMap::new())
            .get(&token)
        {
            Some(transition) => transition.clone(),
            None => {
                let transition = match node.borrow().children.get(&token) {
                    Some(child_node) => child_node.clone(),
                    None => match node.borrow().of_type {
                        ROOT => node.clone(),
                        _ => {
                            let node_failure_suffix = self.get_suffix(&node).failure;
                            self.get_transition(&node_failure_suffix, token)
                        }
                    },
                };
                self.suffix_transitions
                    .get_mut(&node.borrow().index)
                    .unwrap()
                    .insert(token, transition.clone());

                transition
            }
        }
    }
}

impl<'a> Drop for AhoCorasickTrie<'a> {
    fn drop(&mut self) {
        let mut node_queue = VecDeque::<TrieLink>::new();
        node_queue.push_back(self.trie.clone());

        while let Some(current_node) = node_queue.pop_front() {
            let mut current_node_borrow = current_node.borrow_mut();
            current_node_borrow
                .children
                .iter_mut()
                .for_each(|(_, child_node)| node_queue.push_back(child_node.clone()));
            current_node_borrow.suffix = None;
        }
    }
}
