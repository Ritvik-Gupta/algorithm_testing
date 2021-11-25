crate::leetcode::solution!();

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::min;

        if x < 1 {
            return 0;
        }

        let mut guess = f64::from(x / min(x, 2));
        loop {
            let prev_guess = guess;
            guess = (guess + (f64::from(x) / guess)) / 2f64;

            if guess.floor() == prev_guess.floor() {
                return guess as i32;
            }
        }
    }
}
