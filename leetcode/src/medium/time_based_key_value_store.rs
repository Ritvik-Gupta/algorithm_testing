use std::collections::{BTreeMap, HashMap};

#[allow(dead_code)]
struct TimeMap(HashMap<String, BTreeMap<i32, String>>);

impl TimeMap {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(HashMap::new())
    }

    #[allow(dead_code)]
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.0
            .entry(key)
            .or_insert_with(BTreeMap::new)
            .insert(timestamp, value);
    }

    #[allow(dead_code)]
    fn get(&self, key: String, timestamp: i32) -> String {
        self.optional_get(key, timestamp).unwrap_or("".to_string())
    }

    fn optional_get(&self, key: String, timestamp: i32) -> Option<String> {
        Some(
            self.0
                .get(&key)?
                .range(0..=timestamp)
                .next_back()
                .map(|entry| entry.1.clone())?,
        )
    }
}
