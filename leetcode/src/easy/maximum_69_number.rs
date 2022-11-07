crate::solution!();

trait ForEachDigit
where
    Self: Sized,
{
    fn for_each_digit(&self, iteration_func: impl FnMut(Self));
}

impl ForEachDigit for i32 {
    fn for_each_digit(&self, mut iteration_func: impl FnMut(Self)) {
        let mut num = *self;
        while num > 0 {
            let digit = num % 10;
            iteration_func(digit);
            num /= 10;
        }
    }
}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut offset = 1;
        let mut first_six_offset = 0;

        num.for_each_digit(|digit| {
            if digit == 6 {
                first_six_offset = offset;
            }
            offset *= 10;
        });

        num + first_six_offset * (9 - 6)
    }
}
