crate::leetcode::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return Vec::new();
        }

        changed.sort();

        let mut freq_table = HashMap::new();
        changed
            .iter()
            .for_each(|&elm| *freq_table.entry(elm).or_insert(0) += 1);

        let mut result = Vec::with_capacity(changed.len() / 2);
        for &elm in changed.iter() {
            if freq_table[&elm] == 0 {
                continue;
            }

            let double_elm = elm * 2;
            let double_elm_freq = *freq_table.get(&double_elm).unwrap_or(&0);

            if (elm == double_elm && double_elm_freq > 1)
                || (elm != double_elm && double_elm_freq > 0)
            {
                result.push(elm);
                *freq_table.get_mut(&elm).unwrap() -= 1;
                *freq_table.get_mut(&double_elm).unwrap() -= 1;
            }
        }

        if result.len() * 2 != changed.len() {
            return Vec::new();
        }

        result
    }
}
