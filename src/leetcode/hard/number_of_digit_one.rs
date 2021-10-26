pub struct Solution;

static LEVEL_COMBINATIONS: [i32; 9] = [0, 1, 20, 300, 4000, 50000, 600000, 7000000, 80000000];

static LEVEL_OFFSET: [i32; 9] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
];

const MAX_NUMBER: i32 = 1000000000;

impl Solution {
    pub fn count_digit_one(mut num: i32) -> i32 {
        if num == MAX_NUMBER {
            return Solution::count_digit_one(num - 1) + 1;
        }

        let (mut total_digit_one, mut num_ones, mut prev_collected) = (0, 0, 0);
        for digit_level in 0..9 {
            let digit = num % 10;
            num /= 10;

            if digit == 1 {
                num_ones += 1;
            }
            total_digit_one += digit * LEVEL_COMBINATIONS[digit_level]
                + match digit {
                    0 => 0,
                    1 => prev_collected,
                    _ => LEVEL_OFFSET[digit_level],
                };
            prev_collected += digit * LEVEL_OFFSET[digit_level];
        }

        total_digit_one + num_ones
    }
}
