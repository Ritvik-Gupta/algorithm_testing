crate::solution!();

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
        let (mut slow_ptr, mut fast_ptr) = (n as u32, n as u32);
        loop {
            slow_ptr = Solution::sum_of_squares(slow_ptr);
            fast_ptr = Solution::sum_of_squares(Solution::sum_of_squares(fast_ptr));
            if slow_ptr == fast_ptr {
                return slow_ptr == 1;
            }
        }
    }
}
