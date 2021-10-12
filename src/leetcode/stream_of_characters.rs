use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use TrieNodeType::{Token, ROOT};

const DEFAULT_TRIE_TOKEN: char = '_';

pub struct TrieNode {
    depth: usize,
    is_end: bool,
    of_type: TrieNodeType,
    children: BTreeMap<char, TrieLink>,
    suffix: Option<TrieSuffix>,
}

#[derive(Hash, PartialEq, Eq)]
enum TrieNodeType {
    ROOT,
    Token { token: char, parent: TrieLink },
}

#[derive(Clone)]
struct TrieSuffix {
    failure: TrieLink,
    dictionary: Option<TrieLink>,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
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

pub struct TrieLink(Rc<RefCell<TrieNode>>);

impl TrieLink {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(TrieNode::default())))
    }
    pub fn relative_parent_with_token(&self) -> (char, Self) {
        match &self.0.borrow().of_type {
            ROOT => (DEFAULT_TRIE_TOKEN, self.clone()),
            Token { parent, token } => (*token, parent.clone()),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = self.clone();
        for token in word.chars() {
            current_node = {
                let mut current_node_borrow = current_node.0.borrow_mut();
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
        current_node.0.borrow_mut().is_end = true;
    }

    pub fn lock(&self) {
        AhoCorasickTrie::transform(self);
    }
}

impl std::hash::Hash for TrieLink {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl std::cmp::PartialEq for TrieLink {
    fn eq(&self, other: &Self) -> bool {
        *self.0.borrow() == *other.0.borrow()
    }
}

impl std::cmp::Eq for TrieLink {}

impl Clone for TrieLink {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

pub struct AhoCorasickTrie(HashMap<TrieLink, BTreeMap<char, TrieLink>>);

impl AhoCorasickTrie {
    pub fn transform(root: &TrieLink) -> Self {
        use std::collections::VecDeque;

        let mut ac_trie = Self(HashMap::new());
        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.clone());

        while let Some(current_node) = node_queue.pop_front() {
            current_node
                .0
                .borrow()
                .children
                .iter()
                .for_each(|(_, child_node)| node_queue.push_back(child_node.clone()));
            current_node.0.borrow_mut().suffix = Some(ac_trie.get_suffix(&current_node));
        }
        ac_trie.0.clear();

        ac_trie
    }

    fn get_suffix(&mut self, node: &TrieLink) -> TrieSuffix {
        match &node.0.borrow().suffix {
            Some(suffix) => suffix.clone(),
            None => {
                let (relative_token, relative_parent) = node.relative_parent_with_token();
                let failure_suffix = match &relative_parent.0.borrow().of_type {
                    ROOT => relative_parent.clone(),
                    _ => {
                        let parent_failure_suffix = self.get_suffix(&relative_parent).failure;
                        self.get_transition(&parent_failure_suffix, relative_token)
                    }
                };

                let dictionary_suffix = match *failure_suffix.0.borrow() {
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
            .0
            .entry(node.clone())
            .or_insert(BTreeMap::new())
            .get(&token)
        {
            Some(transition) => transition.clone(),
            None => {
                let node_borrow = node.0.borrow();
                let transition = match (node_borrow.children.get(&token), &node_borrow.of_type) {
                    (Some(child_node), _) => child_node.clone(),
                    (None, ROOT) => node.clone(),
                    _ => {
                        let node_failure_suffix = self.get_suffix(node).failure;
                        self.get_transition(&node_failure_suffix, token)
                    }
                };
                self.0
                    .get_mut(node)
                    .unwrap()
                    .insert(token, transition.clone());

                transition
            }
        }
    }
}

struct StreamChecker {
    current_node: TrieLink,
}

impl StreamChecker {
    #[allow(dead_code)]
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieLink::new();
        words.iter().for_each(|word| trie.insert(word));
        trie.lock();
        Self { current_node: trie }
    }

    fn query(&mut self, token: char) -> bool {
        loop {
            self.current_node = {
                let node_borrow = self.current_node.0.borrow();

                if node_borrow.children.contains_key(&token) {
                    break;
                } else if let ROOT = node_borrow.of_type {
                    return false;
                }
                node_borrow.suffix.as_ref().unwrap().failure.clone()
            };
        }

        let child_node = self.current_node.0.borrow().children[&token].clone();
        self.current_node = child_node;

        let node_borrow = self.current_node.0.borrow();
        match node_borrow.suffix.as_ref().unwrap().dictionary.as_ref() {
            Some(_) => true,
            _ => node_borrow.is_end,
        }
    }
}
