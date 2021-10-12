use crate::trie::{TrieLink, TrieNode, TrieNodeType::ROOT, TrieSuffix};
use std::collections::{BTreeMap, HashMap};

pub struct AhoCorasickTrie<'a> {
    trie: &'a mut TrieLink,
    suffix_transitions: HashMap<TrieLink, BTreeMap<char, TrieLink>>,
}

impl<'a> AhoCorasickTrie<'a> {
    pub fn new(trie: &'a mut TrieLink) -> Self {
        use std::collections::VecDeque;

        let mut ac_trie = Self {
            trie,
            suffix_transitions: HashMap::new(),
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

        ac_trie
    }

    pub fn match_against(&self, sentence: &str) {
        let mut current_node = self.trie.clone();

        let sentence = sentence.as_bytes();
        'to_next_token: for (index, &token) in sentence.iter().enumerate() {
            let token = token as char;
            loop {
                current_node = {
                    let node_borrow = current_node.borrow();

                    if node_borrow.children.contains_key(&token) {
                        break;
                    } else if let ROOT = node_borrow.of_type {
                        continue 'to_next_token;
                    }
                    node_borrow.suffix.as_ref().unwrap().failure.clone()
                };
            }

            let mut child_node = current_node.borrow().children[&token].clone();
            if child_node.borrow().is_end {
                let start_idx = index + 1 - child_node.borrow().depth;
                println!(
                    "Match {} at {}",
                    std::str::from_utf8(&sentence[start_idx..index + 1]).unwrap(),
                    index + 1
                );
            }
            current_node = child_node.clone();

            loop {
                child_node = {
                    let node_borrow = child_node.borrow();
                    match &node_borrow.suffix.as_ref().unwrap().dictionary {
                        Some(dictionary_node) => dictionary_node.clone(),
                        None => break,
                    }
                };

                let start_idx = index + 1 - child_node.borrow().depth;
                println!(
                    "Match {} at {}",
                    std::str::from_utf8(&sentence[start_idx..index + 1]).unwrap(),
                    index + 1
                );
            }
        }
    }

    fn get_suffix(&mut self, node: &TrieLink) -> TrieSuffix {
        match &node.borrow().suffix {
            Some(suffix) => suffix.clone(),
            None => {
                let (relative_token, relative_parent) = node.relative_parent_with_token();
                let failure_suffix = match &relative_parent.borrow().of_type {
                    ROOT => relative_parent.clone(),
                    _ => {
                        let parent_failure_suffix = self.get_suffix(&relative_parent).failure;
                        self.get_transition(&parent_failure_suffix, relative_token)
                    }
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
            .entry(node.clone())
            .or_insert(BTreeMap::new())
            .get(&token)
        {
            Some(transition) => transition.clone(),
            None => {
                let node_borrow = node.borrow();
                let transition = match (node_borrow.children.get(&token), &node_borrow.of_type) {
                    (Some(child_node), _) => child_node.clone(),
                    (None, ROOT) => node.clone(),
                    _ => {
                        let node_failure_suffix = self.get_suffix(node).failure;
                        self.get_transition(&node_failure_suffix, token)
                    }
                };
                self.suffix_transitions
                    .get_mut(node)
                    .unwrap()
                    .insert(token, transition.clone());

                transition
            }
        }
    }
}

impl<'a> Drop for AhoCorasickTrie<'a> {
    fn drop(&mut self) {
        use std::collections::VecDeque;

        let mut node_queue = VecDeque::new();
        node_queue.push_back(self.trie.clone());

        while let Some(current_node) = node_queue.pop_front() {
            current_node
                .borrow()
                .children
                .iter()
                .for_each(|(_, child_node)| node_queue.push_back(child_node.clone()));
            current_node.borrow_mut().suffix = None;
        }
    }
}
