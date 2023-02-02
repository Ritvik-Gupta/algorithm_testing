crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        arr.sort();

        let mut map = HashMap::new();
        arr.iter()
            .for_each(|&num| *map.entry(num).or_insert(0) += 1);

        arr.into_iter().for_each(|num| {
            if *map.get(&num).unwrap_or(&0) != 0 && *map.get(&(2 * num)).unwrap_or(&0) != 0 {
                map.entry(2 * num).and_modify(|x| *x -= 1);
                map.entry(num).and_modify(|x| *x -= 1);
            }
        });

        map.iter().all(|(_, &freq)| freq == 0)
    }
}
