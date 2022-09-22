crate::leetcode::solution!();

impl Solution {
    pub fn replace_digits(mut word: String) -> String {
        let word_ptr = word.as_mut_ptr();

        for idx in (1..word.len() as isize).step_by(2) {
            let shifts = unsafe { &mut *word_ptr.offset(idx) };

            *shifts =
                b'a' + (unsafe { *word_ptr.offset(idx - 1) } as u8 - b'a' + (*shifts - b'0')) % 26
        }

        word
    }
}
