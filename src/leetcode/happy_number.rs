pub struct Solution;

impl Solution {
    fn sum_of_squares(mut num: u32) -> u32 {
        let mut sum = 0;
        while num != 0 {
            let rem = num % 10;
            sum += rem * rem;
            num /= 10;
        }
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        let sum_of_sq = Solution::sum_of_squares(n as u32);
        CACHED_RESULTS_FOR_HAPPY_TILL_730[(sum_of_sq - 1) as usize]
    }
}

const CACHED_RESULTS_FOR_HAPPY_TILL_730: [bool; 730] = [
    true, false, false, false, false, false, true, false, false, true, false, false, true, false,
    false, false, false, false, true, false, false, false, true, false, false, false, false, true,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, true, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, true, false, true,
    false, false, false, false, false, false, false, false, true, false, false, true, false, false,
    false, true, false, false, false, false, true, false, false, true, false, false, true, false,
    false, true, false, false, true, false, false, false, false, false, true, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, false, false, true, false, false, false, false, false, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, false, false, false, false, false, false, false, false, true, false, false, false,
    false, false, false, false, false, false, false, false, true, false, true, false, true, true,
    false, false, false, false, false, false, false, false, false, true, false, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, true, false,
    false, false, false, false, false, true, false, false, false, true, false, false, false, false,
    false, true, false, false, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false, true, false, true, false, false, false, false, false, false, false, true, true,
    false, false, false, false, false, false, false, true, false, false, true, false, false, false,
    false, false, true, true, false, false, false, false, false, true, false, false, true, false,
    true, false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, true, false,
    false, false, false, false, true, false, false, true, false, true, true, false, false, false,
    false, false, false, false, true, false, false, true, false, false, false, true, false, false,
    true, false, false, false, false, true, true, false, false, false, false, true, false, false,
    false, false, false, false, true, false, false, false, false, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, false, false,
    false, false, true, false, false, false, false, false, false, false, false, true, false, false,
    false, false, false, false, false, false, true, false, false, true, false, false, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false,
    false, false, false, true, false, true, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, true, false, false, false, false,
    false, false, false, false, true, false, false, false, false, true, true, false, false, false,
    false, false, false, false, false, true, false, false, true, false, true, true, false, false,
    false, false, false, true, false, false, false, false, true, false, false, false, true, false,
    true, true, false, false, false, false, false, false, false, false, true, false, false, false,
    false, false, true, false, true, false, false, false, false, false, false, true, false, false,
    true, false, false, false, false, false, false, false, false, false, false, true, false, false,
    false, false, false, true, false, false, false, false, false, false, false, false, true, false,
    false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,
];
