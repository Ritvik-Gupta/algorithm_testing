use rand::{prelude::ThreadRng, Rng};
use std::collections::{hash_map::Entry::Vacant, HashMap};

struct RandomizedSet {
    keys: Vec<i32>,
    key_frequency_record: HashMap<i32, usize>,
    rng: ThreadRng,
}

impl RandomizedSet {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            key_frequency_record: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> bool {
        if let Vacant(entry) = self.key_frequency_record.entry(val) {
            self.keys.push(val);
            entry.insert(self.keys.len() - 1);
            return true;
        }
        false
    }

    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        self.remove_occurence(val).is_some()
    }

    fn remove_occurence(&mut self, val: i32) -> Option<()> {
        let keys_size = self.keys.len();
        let removed_key_idx = self.key_frequency_record.remove(&val)?;
        if removed_key_idx != keys_size - 1 {
            *self.key_frequency_record.get_mut(self.keys.last()?)? = removed_key_idx;
            self.keys.swap(removed_key_idx, keys_size - 1);
        }
        self.keys.pop();
        Some(())
    }

    #[allow(dead_code)]
    fn get_random(&mut self) -> i32 {
        self.keys[self.rng.gen::<usize>() % self.keys.len()]
    }
}
