struct LockingNode {
    parent_idx: i32,
    locked_by: Option<i32>,
    children_indices: Vec<i32>,
}

struct LockingTree(Vec<LockingNode>);

impl std::ops::Index<i32> for LockingTree {
    type Output = LockingNode;

    fn index(&self, idx: i32) -> &Self::Output {
        &self.0[idx as usize]
    }
}

impl std::ops::IndexMut<i32> for LockingTree {
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        &mut self.0[idx as usize]
    }
}

impl LockingTree {
    #[allow(dead_code)]
    fn new(parent: Vec<i32>) -> Self {
        let mut locking_tree = Self(
            parent
                .iter()
                .map(|&parent_idx| LockingNode {
                    parent_idx,
                    locked_by: None,
                    children_indices: Vec::new(),
                })
                .collect::<Vec<_>>(),
        );

        parent
            .into_iter()
            .enumerate()
            .for_each(|(idx, parent_idx)| {
                if parent_idx != -1 {
                    locking_tree[parent_idx].children_indices.push(idx as i32);
                }
            });

        locking_tree
    }

    #[allow(dead_code)]
    fn lock(&mut self, num: i32, user: i32) -> bool {
        let node_lock = &mut self[num].locked_by;
        match node_lock {
            Some(_) => false,
            _ => {
                *node_lock = Some(user);
                true
            }
        }
    }

    #[allow(dead_code)]
    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let node_lock = &mut self[num].locked_by;
        match node_lock {
            Some(by_user) if *by_user == user => {
                *node_lock = None;
                true
            }
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        use std::collections::LinkedList;

        if !self.has_all_unlocked_ancestors(num) || !self.has_any_locked_descendant(num) {
            return false;
        }

        let mut node_queue = LinkedList::new();
        node_queue.push_back(num);
        while let Some(node_idx) = node_queue.pop_front() {
            self[node_idx].locked_by = None;
            node_queue.extend(self[node_idx].children_indices.iter());
        }
        self[num].locked_by = Some(user);

        true
    }

    fn has_any_locked_descendant(&self, num: i32) -> bool {
        if self[num].locked_by.is_some() {
            return true;
        }
        self[num]
            .children_indices
            .iter()
            .any(|&child_idx| self.has_any_locked_descendant(child_idx))
    }

    fn has_all_unlocked_ancestors(&self, num: i32) -> bool {
        if num == -1 {
            return true;
        }
        if self[num].locked_by.is_some() {
            return false;
        }
        self.has_all_unlocked_ancestors(self[num].parent_idx)
    }
}
