crate::solution!();

fn as_index(token: char) -> usize {
    (token as u8 - b'a') as usize
}

#[derive(Clone)]
struct TrieNode(Vec<Option<Box<TrieNode>>>);

impl TrieNode {
    fn new() -> Box<Self> {
        Box::new(Self(vec![None; 26]))
    }

    fn construct_from(words: Vec<String>) -> Box<Self> {
        let mut trie_root = TrieNode::new();

        for word in words {
            let mut curr_node = &mut trie_root;

            for token in word.chars().rev() {
                let link = &mut curr_node.0[as_index(token)];

                curr_node = match link {
                    Some(child_node) => child_node,
                    _ => {
                        *link = Some(TrieNode::new());
                        link.as_mut().unwrap()
                    }
                };
            }
        }

        trie_root
    }

    fn rcr_find_chain_length(&self, total_chain: &mut i32, curr_len: i32) {
        let mut has_any_child = false;

        self.0
            .iter()
            .filter_map(|x| x.as_ref())
            .for_each(|child_node| {
                has_any_child = true;
                child_node.rcr_find_chain_length(total_chain, curr_len + 1)
            });

        if !has_any_child {
            *total_chain += curr_len;
        }
    }

    fn collect_chain_lengths(&self) -> i32 {
        let mut total_chain = 0;
        self.rcr_find_chain_length(&mut total_chain, 1);
        total_chain
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        TrieNode::construct_from(words).collect_chain_lengths()
    }
}
