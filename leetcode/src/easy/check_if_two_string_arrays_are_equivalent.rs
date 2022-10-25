crate::solution!();

use std::iter::Peekable;

fn check_zip_equal(
    mut iter1: Peekable<impl Iterator<Item = char>>,
    mut iter2: Peekable<impl Iterator<Item = char>>,
) -> Option<()> {
    loop {
        if iter1.peek().is_none() && iter2.peek().is_none() {
            return Some(());
        } else if iter1.next()? != iter2.next()? {
            return None;
        }
    }
}

impl Solution {
    pub fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
        check_zip_equal(
            words1.iter().flat_map(|word| word.chars()).peekable(),
            words2.iter().flat_map(|word| word.chars()).peekable(),
        )
        .is_some()
    }
}
