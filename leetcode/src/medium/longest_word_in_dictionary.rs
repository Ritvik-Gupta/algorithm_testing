crate::solution!();

use std::{cmp::Reverse, collections::VecDeque};

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as _
}

fn idx_to_ch(idx: usize) -> u8 {
    idx as u8 + b'a'
}

#[derive(Default)]
struct Trie {
    is_checkpoint: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for ch in word.chars() {
            let child_node = &mut node.children[ch_to_idx(ch)];
            node = match child_node {
                Some(child_node) => child_node,
                None => {
                    *child_node = Some(Trie::new());
                    child_node.as_mut().unwrap()
                }
            };
        }
        node.is_checkpoint = true;
    }

    fn traverse_checkpoint_path(&self, path: VecDeque<u8>) -> VecDeque<u8> {
        self.children
            .iter()
            .enumerate()
            .filter_map(|(idx, node)| {
                let node = node.as_ref()?;
                node.is_checkpoint.then(|| (idx, node))
            })
            .map(|(idx, node)| {
                let mut child_path = node.traverse_checkpoint_path(path.clone());
                child_path.push_front(idx_to_ch(idx));
                child_path
            })
            .max_by_key(|path| (path.len(), Reverse(path.clone())))
            .unwrap_or(path)
    }
}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        let max_word_size = words
            .into_iter()
            .map(|word| {
                let size = word.len();
                trie.insert(word);
                size
            })
            .max()
            .unwrap();
        String::from_utf8_lossy(
            trie.traverse_checkpoint_path(VecDeque::with_capacity(max_word_size))
                .make_contiguous(),
        )
        .to_string()
    }
}
