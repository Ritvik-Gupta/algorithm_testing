crate::solution!();

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut unequal_tokens = None;
        let mut num_swaps = 0;

        for (ch1, ch2) in s1.chars().zip(s2.chars()) {
            if ch1 != ch2 {
                match unequal_tokens {
                    Some((ut1, ut2)) if ut1 == ch2 && ut2 == ch1 => {
                        num_swaps += 1;
                        unequal_tokens = None;
                    }
                    Some(_) => return false,
                    None => unequal_tokens = Some((ch1, ch2)),
                }
            }
        }

        (0..=1).contains(&num_swaps) && unequal_tokens.is_none()
    }
}
