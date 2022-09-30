crate::solution!();

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let (mut subsequences, mut max_subsequence_size) = (std::collections::BTreeMap::new(), 0);

        for elm in arr {
            subsequences.insert(elm, subsequences.get(&(elm - difference)).unwrap_or(&0) + 1);
            if subsequences[&elm] > max_subsequence_size {
                max_subsequence_size = subsequences[&elm];
            }
        }

        max_subsequence_size
    }
}
