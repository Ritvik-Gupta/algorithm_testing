crate::solution!();

fn reverse(mut num: i32) -> i32 {
    let mut rev_num = 0;
    while num > 0 {
        rev_num = rev_num * 10 + num % 10;
        num /= 10;
    }
    rev_num
}

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        for x in num / 2..=num {
            if x + reverse(x) == num {
                return true;
            }
        }
        false
    }
}
