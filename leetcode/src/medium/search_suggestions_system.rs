crate::solution!();

fn as_index(token: char) -> usize {
    (token as u8 - b'a') as usize
}

#[derive(Clone)]
struct TrieNode {
    end_word: Option<String>,
    links: Vec<Option<Box<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Box<Self> {
        Box::new(Self {
            end_word: None,
            links: vec![None; 26],
        })
    }

    fn construct_from(words: Vec<String>) -> Box<Self> {
        let mut trie_root = TrieNode::new();
        words.into_iter().for_each(|word| trie_root.add_word(word));
        trie_root
    }

    fn add_word(&mut self, word: String) {
        let mut curr_node = self;

        for token in word.chars() {
            let link = &mut curr_node.links[as_index(token)];

            curr_node = match link {
                Some(child_node) => child_node,
                _ => {
                    *link = Some(TrieNode::new());
                    link.as_mut().unwrap()
                }
            };
        }

        curr_node.end_word = Some(word);
    }

    fn recursive_search(self: &Box<Self>, closest_searches: &mut Vec<String>) {
        if closest_searches.len() == closest_searches.capacity() {
            return;
        }

        if let Some(word) = &self.end_word {
            closest_searches.push(word.clone());
        }

        self.links
            .iter()
            .filter_map(Option::as_ref)
            .for_each(|child_node| child_node.recursive_search(closest_searches));
    }

    fn search_closest_words(
        self: &Box<Self>,
        search_word: &String,
        total_closest_searches: usize,
    ) -> Vec<Vec<String>> {
        let mut result = Vec::with_capacity(search_word.len());

        let mut curr_node = self;
        for token in search_word.chars() {
            curr_node = match &curr_node.links[as_index(token)] {
                Some(child_node) => child_node,
                _ => return result,
            };

            let mut closest_searches = Vec::with_capacity(total_closest_searches);
            curr_node.recursive_search(&mut closest_searches);
            result.push(closest_searches);
        }
        result
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut suggested_prods =
            TrieNode::construct_from(products).search_closest_words(&search_word, 3);
        suggested_prods
            .extend(std::iter::repeat(Vec::new()).take(search_word.len() - suggested_prods.len()));

        suggested_prods
    }
}
