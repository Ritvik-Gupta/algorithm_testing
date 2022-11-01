use std::collections::HashMap;

struct FindSumPairs {
    const_store: HashMap<i32, i32>,
    mutable_store: HashMap<i32, i32>,
    lookback_table: Vec<i32>,
}

#[inline]
fn create_freq_store(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut freq_store = HashMap::new();
    nums.iter()
        .for_each(|&elm| *freq_store.entry(elm).or_default() += 1);
    freq_store
}

impl FindSumPairs {
    #[allow(dead_code)]
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Self {
            const_store: create_freq_store(&nums1),
            mutable_store: create_freq_store(&nums2),
            lookback_table: nums2,
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, index: i32, adding_val: i32) {
        let val = &mut self.lookback_table[index as usize];
        *self.mutable_store.entry(*val).or_default() -= 1;

        *val += adding_val;
        *self.mutable_store.entry(*val).or_default() += 1;
    }

    #[allow(dead_code)]
    fn count(&self, total_sum: i32) -> i32 {
        self.const_store
            .iter()
            .map(|(&num, &const_freq)| {
                const_freq
                    * *self
                        .mutable_store
                        .get(&total_sum.wrapping_sub(num))
                        .unwrap_or(&0)
            })
            .sum()
    }
}
