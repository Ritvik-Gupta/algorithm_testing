crate::solution!();

fn is_palindromic(str: &String) -> bool {
    let size = str.len() as isize;
    let ptr = str.as_ptr();
    (0..size / 2).all(
        |i| unsafe { *ptr.wrapping_offset(i) } == unsafe { *ptr.wrapping_offset(size - 1 - i) },
    )
}

impl Solution {
    pub fn remove_palindrome_sub(str: String) -> i32 {
        2 - if is_palindromic(&str) { 1 } else { 0 }
    }
}
