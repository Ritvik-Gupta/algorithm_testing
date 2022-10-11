use std::collections::{BTreeMap, HashMap, HashSet};

struct StockPrice {
    timestamp_record: HashMap<i32, i32>,
    price_record: BTreeMap<i32, HashSet<i32>>,
    latest_timestamp: i32,
}

impl StockPrice {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            timestamp_record: HashMap::new(),
            price_record: BTreeMap::new(),
            latest_timestamp: 0,
        }
    }

    #[allow(dead_code)]
    fn update(&mut self, timestamp: i32, price: i32) {
        if let Some(&prev_price) = self.timestamp_record.get(&timestamp) {
            let cluster = self.price_record.get_mut(&prev_price).unwrap();
            cluster.remove(&timestamp);
            if cluster.is_empty() {
                self.price_record.remove(&prev_price);
            }
        }

        self.price_record
            .entry(price)
            .or_insert_with(HashSet::new)
            .insert(timestamp);

        self.timestamp_record.insert(timestamp, price);
        self.latest_timestamp = i32::max(self.latest_timestamp, timestamp);
    }

    #[allow(dead_code)]
    fn current(&self) -> i32 {
        self.timestamp_record[&self.latest_timestamp]
    }

    #[allow(dead_code)]
    fn maximum(&self) -> i32 {
        *self.price_record.keys().next_back().unwrap()
    }

    #[allow(dead_code)]
    fn minimum(&self) -> i32 {
        *self.price_record.keys().next().unwrap()
    }
}
