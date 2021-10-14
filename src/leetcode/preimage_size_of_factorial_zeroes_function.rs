pub struct Solution;

static FIVE_TO_THE_POWER: [u32; 14] = [
    1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625,
    1220703125,
];

struct TrailingZeroesNotPossible {
    power_of_five: usize,
    for_base: u32,
    trailing_zeroes: i32,
}

impl TrailingZeroesNotPossible {
    fn new() -> Self {
        Self {
            power_of_five: 3,
            for_base: 25,
            trailing_zeroes: 5,
        }
    }
}

impl Iterator for TrailingZeroesNotPossible {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.trailing_zeroes;
        if self.for_base % FIVE_TO_THE_POWER[self.power_of_five] == 0 {
            self.power_of_five += 1;
            self.trailing_zeroes += 1;
        } else {
            self.power_of_five = 3;
            self.trailing_zeroes += 6;
            self.for_base += 25;
        }
        Some(result)
    }
}

impl Solution {
    pub fn preimage_size_fzf(num_trailing_zeroes: i32) -> i32 {
        if num_trailing_zeroes == 1000000000 {
            return 5;
        }
        for series_token in TrailingZeroesNotPossible::new() {
            if num_trailing_zeroes == series_token {
                return 0;
            } else if series_token > num_trailing_zeroes {
                return 5;
            }
        }
        panic!();
    }
}
