crate::solution!();

fn split_digits(mut num: i32) -> Vec<i32> {
    let mut digits = Vec::new();

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits
}

fn digit_checking_case(num: i32) -> bool {
    let digits = split_digits(num);

    let mut has_carry = false;
    for i in 0..digits.len() / 2 {
        let palin_digit = digits[digits.len() - 1 - i] + if has_carry { 10 } else { 0 };

        has_carry = match palin_digit - digits[i] {
            0 => false,
            1 => true,
            _ => return false,
        };
    }

    if digits.len() % 2 == 0 {
        !has_carry
    } else {
        digits[digits.len() / 2] % 2 == 0
    }
}

fn is_palindrome(mut n: i32) -> bool {
    let num = n;
    let mut rev_num = 0;
    while n > 0 {
        rev_num = rev_num * 10 + n % 10;
        n /= 10;
    }
    num == rev_num
}

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        digit_checking_case(num) | (num % 2 == 0 && is_palindrome(num / 2))
    }
}
