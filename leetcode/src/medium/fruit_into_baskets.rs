crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let (mut i, mut j) = (0, 0);

        while j < fruits.len() {
            *count.entry(fruits[j]).or_insert(0) += 1;
            if count.len() > 2 {
                *count.entry(fruits[i]).or_insert(0) -= 1;
                if count[&fruits[i]] == 0 {
                    count.remove(&fruits[i]);
                }
                i += 1;
            }
            j += 1;
        }
        (j - i) as i32
    }
}
