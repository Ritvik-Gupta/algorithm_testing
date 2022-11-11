crate::solution!();

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or_else(|| -1, |pos| pos as i32)
    }
}
