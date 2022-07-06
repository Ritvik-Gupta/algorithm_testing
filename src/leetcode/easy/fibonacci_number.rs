crate::leetcode::solution!();

impl Solution {
    pub fn fib(mut num: i32) -> i32 {
        if num <= 1 {
            return num;
        }

        let (mut a, mut b) = (0, 1);

        while num > 1 {
            let sum = a + b;
            a = b;
            b = sum;

            num -= 1;
        }
        b
    }
}
