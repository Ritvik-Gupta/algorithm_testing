use std::collections::{BTreeMap, HashMap};

#[allow(dead_code)]
struct SnapshotArray {
    index_snapshots: HashMap<i32, BTreeMap<i32, i32>>,
    snap_identifier: i32,
}

impl SnapshotArray {
    #[allow(dead_code)]
    fn new(_length: i32) -> Self {
        Self {
            index_snapshots: HashMap::new(),
            snap_identifier: 0,
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, index: i32, val: i32) {
        self.index_snapshots
            .entry(index)
            .or_insert(BTreeMap::new())
            .insert(self.snap_identifier, val);
    }

    #[allow(dead_code)]
    fn snap(&mut self) -> i32 {
        let snap_id = self.snap_identifier;
        self.snap_identifier += 1;
        snap_id
    }

    #[allow(dead_code)]
    fn get(&mut self, index: i32, snap_id: i32) -> i32 {
        let snapshots = self.index_snapshots.entry(index).or_insert(BTreeMap::new());
        snapshots
            .range(..=snap_id)
            .next_back()
            .map_or(0, |(_, &val)| val)
    }
}
