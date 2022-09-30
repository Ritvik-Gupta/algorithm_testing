use rand::{prelude::ThreadRng, Rng};
use std::collections::{
    btree_map::Entry::{Occupied, Vacant},
    BTreeMap, BinaryHeap,
};

struct RandomizedCollection {
    keys: Vec<i32>,
    key_frequency_record: BTreeMap<i32, BinaryHeap<usize>>,
    random_generator: ThreadRng,
}

impl RandomizedCollection {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            key_frequency_record: BTreeMap::new(),
            random_generator: rand::thread_rng(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> bool {
        self.keys.push(val);
        let key_freq_entry = self.key_frequency_record.entry(val);
        let did_contains_val = match key_freq_entry {
            Vacant(_) => true,
            Occupied(_) => false,
        };
        key_freq_entry.or_default().push(self.keys.len() - 1);
        did_contains_val
    }

    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        self.remove_one_occurence(val).is_some()
    }

    fn remove_one_occurence(&mut self, val: i32) -> Option<()> {
        match self.key_frequency_record.entry(val) {
            Occupied(mut entry) => {
                let removed_key_idx = entry.get_mut().pop()?;
                if entry.get().is_empty() {
                    entry.remove_entry();
                }
                if removed_key_idx != self.keys.len() - 1 {
                    let last_key_heap = self.key_frequency_record.get_mut(self.keys.last()?)?;
                    let last_key_idx = last_key_heap.pop()?;
                    last_key_heap.push(removed_key_idx);
                    self.keys.swap(removed_key_idx, last_key_idx);
                }
                self.keys.pop();
                Some(())
            }
            Vacant(_) => None,
        }
    }

    #[allow(dead_code)]
    fn get_random(&mut self) -> i32 {
        self.keys[self.random_generator.gen::<usize>() % self.keys.len()]
    }
}
