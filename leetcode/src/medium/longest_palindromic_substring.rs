crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let word = s.as_bytes();
        let mut substring = &word[0..1];

        let mut palindromic_substrings = HashSet::with_capacity(word.len() * (word.len() + 1) / 2);

        for i in 0..word.len() {
            palindromic_substrings.insert((i, i));
        }

        for i in (0..word.len()).rev() {
            for j in i + 1..word.len() {
                if word[i] == word[j]
                    && (j == i + 1 || palindromic_substrings.contains(&(i + 1, j - 1)))
                {
                    palindromic_substrings.insert((i, j));

                    if j - i + 1 > substring.len() {
                        substring = &word[i..=j];
                    }
                }
            }
        }

        String::from_utf8_lossy(substring).into_owned()
    }
}
