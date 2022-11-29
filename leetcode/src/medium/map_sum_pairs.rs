use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    store: i32,
    kids: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn recursive_sum(&self) -> i32 {
        self.store
            + self
                .kids
                .values()
                .map(|node| node.recursive_sum())
                .sum::<i32>()
    }
}

struct MapSum(TrieNode);

impl MapSum {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(TrieNode::default())
    }

    #[allow(dead_code)]
    fn insert(&mut self, key: String, val: i32) {
        key.chars()
            .fold(&mut self.0, |node, ch| node.kids.entry(ch).or_default())
            .store = val;
    }

    #[allow(dead_code)]
    fn sum(&self, prefix: String) -> i32 {
        prefix
            .chars()
            .try_fold(&self.0, |node, ch| node.kids.get(&ch))
            .map_or(0, |node| node.recursive_sum())
    }
}
