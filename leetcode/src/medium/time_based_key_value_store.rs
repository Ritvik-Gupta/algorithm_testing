use std::collections::HashMap;

struct TimedValue {
    timestamp: i32,
    value: String,
}

struct TimeMap(HashMap<String, Vec<TimedValue>>);

impl TimeMap {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(HashMap::new())
    }

    #[allow(dead_code)]
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.0
            .entry(key)
            .or_insert_with(Vec::new)
            .push(TimedValue { timestamp, value });
    }

    #[allow(dead_code)]
    fn get(&self, key: String, timestamp: i32) -> String {
        self.optional_get(key, timestamp).unwrap_or("".to_string())
    }

    fn optional_get(&self, key: String, timestamp: i32) -> Option<String> {
        let cluster = self.0.get(&key)?;

        let index = cluster
            .binary_search_by_key(&timestamp, |val| val.timestamp)
            .map_or_else(|i| i.wrapping_sub(1), |i| i);

        Some(cluster.get(index)?.value.clone())
    }
}
