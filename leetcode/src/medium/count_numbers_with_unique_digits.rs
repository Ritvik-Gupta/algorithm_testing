crate::solution!();

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        1 + (1..=n)
            .scan(1, |st, x| {
                *st *= match x {
                    1 => 9,
                    _ => 9 - x + 2,
                };
                Some(*st)
            })
            .sum::<i32>()
    }
}
