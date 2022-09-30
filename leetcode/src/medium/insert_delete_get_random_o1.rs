use rand::{prelude::ThreadRng, Rng};
use std::collections::{
    btree_map::Entry::{Occupied, Vacant},
    BTreeMap,
};

struct RandomizedSet {
    keys: Vec<i32>,
    key_frequency_record: BTreeMap<i32, usize>,
    random_generator: ThreadRng,
}

impl RandomizedSet {
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
        match self.key_frequency_record.entry(val) {
            Occupied(_) => false,
            Vacant(entry) => {
                self.keys.push(val);
                entry.insert(self.keys.len() - 1);
                true
            }
        }
    }

    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        self.remove_occurence(val).is_some()
    }

    fn remove_occurence(&mut self, val: i32) -> Option<()> {
        let removed_key_idx = self.key_frequency_record.remove(&val)?;
        let keys_size = self.keys.len();
        if removed_key_idx != keys_size - 1 {
            *self.key_frequency_record.get_mut(self.keys.last()?)? = removed_key_idx;
            self.keys.swap(removed_key_idx, keys_size - 1);
        }
        self.keys.pop();
        Some(())
    }

    #[allow(dead_code)]
    fn get_random(&mut self) -> i32 {
        self.keys[self.random_generator.gen::<usize>() % self.keys.len()]
    }
}
