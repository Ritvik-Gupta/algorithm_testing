crate::solution!();

fn digits(mut num: i32) -> impl Iterator<Item = i32> {
    (0..).map_while(move |_| {
        if num == 0 {
            return None;
        }
        let digit = num % 10;
        num /= 10;
        Some(digit)
    })
}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        digits(num).filter(|&digit| num % digit == 0).count() as i32
    }
}
