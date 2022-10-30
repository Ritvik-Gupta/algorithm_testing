crate::solution!();

fn sum(mut num: i64) -> i32 {
    (0..)
        .map_while(|_| match num {
            0 => None,
            _ => {
                let digit = num % 10;
                num /= 10;
                Some(digit as i32)
            }
        })
        .sum()
}

impl Solution {
    pub fn make_integer_beautiful(mut num: i64, target: i32) -> i64 {
        let (initial_num, mut base) = (num, 1);

        while sum(num) > target {
            num = num / 10 + 1;
            base *= 10;
        }
        num * base - initial_num
    }
}
