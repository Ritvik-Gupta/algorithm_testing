pub struct Solution;

enum DigitRotationResult {
    Invalid,
    SelfMap,
    Valid,
}

use DigitRotationResult::{Invalid, SelfMap, Valid};

static DIGITS_ROTATION: [DigitRotationResult; 10] = [
    SelfMap, SelfMap, Valid, Invalid, Invalid, Valid, Valid, Invalid, SelfMap, Valid,
];

impl Solution {
    fn contains_valid_rotation_digit(mut num: usize) -> bool {
        let mut can_be_rotated = false;
        while num > 0 {
            let digit = num % 10;
            match DIGITS_ROTATION[digit] {
                Invalid => return false,
                Valid => can_be_rotated = true,
                SelfMap => {}
            }
            num /= 10;
        }
        can_be_rotated
    }

    pub fn rotated_digits(n: i32) -> i32 {
        let num = n as usize;

        (1..=num)
            .filter(|&num| Solution::contains_valid_rotation_digit(num))
            .count() as i32
    }
}
