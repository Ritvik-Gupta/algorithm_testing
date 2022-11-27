crate::solution!();

use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
struct DefaultMap<K, V>(HashMap<K, V>, V)
where
    K: Hash + PartialEq + Eq,
    V: Default;

impl<K, V> DefaultMap<K, V>
where
    K: Hash + PartialEq + Eq,
    V: Default,
{
    fn new() -> Self {
        Self(HashMap::new(), V::default())
    }
}

impl<K, V> std::ops::Index<K> for DefaultMap<K, V>
where
    K: Hash + PartialEq + Eq,
    V: Default,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.0.get(&index).unwrap_or_else(|| &self.1)
    }
}

impl<K, V> std::ops::IndexMut<K> for DefaultMap<K, V>
where
    K: Hash + PartialEq + Eq,
    V: Default,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.0.entry(index).or_insert(V::default())
    }
}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![DefaultMap::<i64, i32>::new(); n];

        let mut total_subseq = 0;
        for (i, a) in nums.iter().map(|&x| x as i64).enumerate() {
            for (j, b) in nums[..i].iter().map(|&x| x as i64).enumerate() {
                dp[i][a - b] += dp[j][a - b] + 1;
                total_subseq += dp[j][a - b];
            }
        }
        total_subseq
    }
}
