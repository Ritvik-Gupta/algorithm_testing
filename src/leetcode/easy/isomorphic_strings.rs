crate::leetcode::solution!();

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut lookup_table_s = std::collections::BTreeMap::new();
        let mut lookup_table_t = std::collections::BTreeMap::new();
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if *lookup_table_s.entry(ch_s).or_insert(ch_t) != ch_t
                || *lookup_table_t.entry(ch_t).or_insert(ch_s) != ch_s
            {
                return false;
            }
        }
        true
    }
}
