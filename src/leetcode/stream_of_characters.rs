const TOTAL_ALPHABETS: usize = 26;
const ALPHABET_OFFSET: u8 = b'a';

struct TrieNode {
    children: [Option<Box<TrieNode>>; TOTAL_ALPHABETS],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Box<Self> {
        Box::new(Self {
            children: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            is_end: false,
        })
    }
}

use std::collections::LinkedList;

struct StreamChecker {
    root: Box<TrieNode>,
    read_chars: LinkedList<u8>,
    max_depth: usize,
}

impl StreamChecker {
    #[allow(dead_code)]
    fn new(words: Vec<String>) -> Self {
        let mut trie_root = TrieNode::new();
        let mut max_depth = 0;

        for word in words {
            if word.len() > max_depth {
                max_depth = word.len();
            }

            let mut current_node = &mut trie_root;
            for token in word
                .bytes()
                .rev()
                .map(|token| (token - ALPHABET_OFFSET) as usize)
            {
                if let None = current_node.children[token] {
                    current_node.children[token] = Some(TrieNode::new());
                }
                current_node = current_node.children[token].as_mut().unwrap();
            }
            current_node.is_end = true;
        }

        Self {
            root: trie_root,
            read_chars: LinkedList::new(),
            max_depth,
        }
    }

    #[allow(dead_code)]
    fn query(&mut self, letter: char) -> bool {
        self.read_chars.push_back(letter as u8);
        if self.read_chars.len() > self.max_depth {
            self.read_chars.pop_front();
        }

        let mut current_node = &self.root;
        for token in self
            .read_chars
            .iter()
            .rev()
            .map(|&token| (token - ALPHABET_OFFSET) as usize)
        {
            current_node = match current_node.children[token].as_ref() {
                Some(child_node) => child_node,
                _ => break,
            };
            if current_node.is_end {
                return true;
            }
        }
        false
    }
}
