fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

#[derive(Default)]
struct Trie {
    is_checkpoint: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    #[allow(dead_code)]
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut node = self;

        for ch in word.chars() {
            let child_node = &mut node.children[ch_to_idx(ch)];
            match child_node {
                Some(child_node) => node = child_node,
                None => {
                    *child_node = Some(Self::new());
                    node = child_node.as_mut().unwrap();
                }
            }
        }
        node.is_checkpoint = true;
    }

    fn traverse_path(&self, word: String) -> Option<&Self> {
        let mut node = self;
        for ch in word.chars() {
            node = node.children[ch_to_idx(ch)].as_ref()?;
        }
        Some(node)
    }

    fn recur_to_checkpoint(&self) -> bool {
        self.is_checkpoint
            || self
                .children
                .iter()
                .filter_map(|node| node.as_ref())
                .any(|node| node.recur_to_checkpoint())
    }

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        self.traverse_path(word)
            .map_or(false, |node| node.is_checkpoint)
    }

    #[allow(dead_code)]
    fn starts_with(&self, prefix: String) -> bool {
        self.traverse_path(prefix)
            .map_or(false, |node| node.recur_to_checkpoint())
    }
}
