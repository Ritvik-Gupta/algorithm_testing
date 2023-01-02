crate::solution!();

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        match a == b {
            true => -1,
            _ => usize::max(a.len(), b.len()) as i32,
        }
    }
}
