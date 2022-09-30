crate::solution!();

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
            match DIGITS_ROTATION[num % 10] {
                Invalid => return false,
                Valid => can_be_rotated = true,
                SelfMap => {}
            }
            num /= 10;
        }
        can_be_rotated
    }

    pub fn rotated_digits(n: i32) -> i32 {
        (1..=n as usize)
            .filter(|&num| Solution::contains_valid_rotation_digit(num))
            .count() as i32
    }
}
