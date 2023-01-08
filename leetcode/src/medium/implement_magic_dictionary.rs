use std::str::Chars;

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

#[derive(Default)]
struct MagicDictionary {
    is_checkpoint: bool,
    children: [Option<Box<MagicDictionary>>; 26],
}

impl MagicDictionary {
    #[allow(dead_code)]
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    #[allow(dead_code)]
    fn build_dict(&mut self, dictionary: Vec<String>) {
        dictionary.into_iter().for_each(|word| self.insert(word));
    }

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

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        self.magic_search(word).is_some()
    }

    fn magic_search(&self, word: String) -> Option<()> {
        let mut word = word.chars();
        let mut node = self;
        while let Some(ch) = word.next() {
            if (0..26)
                .filter_map(|idx| {
                    (idx != ch_to_idx(ch))
                        .then(|| node.children[idx].as_ref())
                        .flatten()
                })
                .any(|node| node.normal_search(word.clone()))
            {
                return Some(());
            }

            node = node.children[ch_to_idx(ch)].as_ref()?;
        }
        None
    }

    fn normal_search(&self, word: Chars) -> bool {
        let mut node = self;
        for ch in word {
            match &node.children[ch_to_idx(ch)] {
                Some(child_node) => node = child_node,
                None => return false,
            }
        }
        node.is_checkpoint
    }
}
