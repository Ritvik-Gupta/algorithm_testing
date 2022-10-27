crate::solution!();

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let is_self_dividing_num = |mut num| {
            let dividend = num;
            (0..)
                .map_while(|_| match num {
                    0 => None,
                    _ => {
                        let digit = num % 10;
                        num /= 10;
                        Some(digit)
                    }
                })
                .all(|digit| digit != 0 && dividend % digit == 0)
        };

        (left..=right)
            .filter(|&num| is_self_dividing_num(num))
            .collect()
    }
}
