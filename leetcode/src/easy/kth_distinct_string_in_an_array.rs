crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut freq_table = HashMap::new();

        arr.iter()
            .for_each(|word| *freq_table.entry(word.clone()).or_insert(0) += 1);

        for word in arr.iter().filter(|&word| freq_table[word] == 1) {
            k -= 1;
            if k == 0 {
                return word.clone();
            }
        }

        "".to_string()
    }
}
