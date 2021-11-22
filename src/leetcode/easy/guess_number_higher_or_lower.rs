pub struct Solution;

unsafe fn guess(_num: i32) -> i32 {
    unimplemented!();
}

impl Solution {
    #[allow(non_snake_case)]
    pub unsafe fn guessNumber(n: i32) -> i32 {
        let (mut lower_bound, mut upper_bound) = (1, n);
        while lower_bound <= upper_bound {
            let mid_guess = lower_bound + (upper_bound - lower_bound) / 2;
            match guess(mid_guess) {
                -1 => upper_bound = mid_guess - 1,
                1 => lower_bound = mid_guess + 1,
                _ => return mid_guess,
            }
        }
        unreachable!();
    }
}
