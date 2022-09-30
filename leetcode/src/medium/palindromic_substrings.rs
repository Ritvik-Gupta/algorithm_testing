crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn count_substrings(word: String) -> i32 {
        let word_ptr = word.as_ptr();
        let word_size = word.len() as isize;

        let mut palindromic_substrings = HashSet::with_capacity(word.len());

        for i in 0..word_size {
            palindromic_substrings.insert((i, i));
        }

        for i in (0..word_size).rev() {
            for j in i + 1..word_size {
                if unsafe { *word_ptr.wrapping_offset(i) == *word_ptr.wrapping_offset(j) }
                    && (j == i + 1 || palindromic_substrings.contains(&(i + 1, j - 1)))
                {
                    palindromic_substrings.insert((i, j));
                }
            }
        }

        palindromic_substrings.len() as i32
    }
}
