crate::solution!();

impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        let len = palindrome.len() as isize;

        if len == 1 {
            return "".to_string();
        }

        let ptr = palindrome.as_mut_ptr();

        for i in 0..len / 2 {
            if unsafe { *ptr.offset(i) } != b'a' {
                *unsafe { &mut *ptr.offset(i) } = b'a';
                return palindrome;
            }
        }

        *unsafe { &mut *ptr.offset(len - 1) } = b'b';
        palindrome
    }
}
