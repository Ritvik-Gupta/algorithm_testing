use crate::trie::link::TrieLink;

impl std::fmt::Debug for TrieLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {} ]", self.borrow().index)
    }
}

impl TrieLink {
    pub fn print(&self) {
        use std::collections::VecDeque;

        let mut node_queue = VecDeque::new();
        node_queue.push_back(self.clone());

        while let Some(current_node) = node_queue.pop_front() {
            current_node
                .borrow()
                .children
                .iter()
                .for_each(|(_, child_node)| node_queue.push_back(child_node.clone()));

            println!("\n{:#?}\n", current_node.borrow());
        }
    }
}
