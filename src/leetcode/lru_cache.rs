const LRU_CACHE_MAX_SIZE: usize = 10000;

struct LRUCache<'a> {
    cache: &'a [i32],
}

impl<'a> LRUCache<'a> {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: &[-1; LRU_CACHE_MAX_SIZE],
        }
    }

    fn get(&self, key: i32) -> i32 {
        panic!()
    }

    fn put(&mut self, key: i32, value: i32) {
        panic!()
    }
}
