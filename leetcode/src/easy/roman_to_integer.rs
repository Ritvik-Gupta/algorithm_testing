crate::solution!();

fn roman_value(token: u8) -> i32 {
    match token {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn roman_to_int(roman: String) -> i32 {
        let roman = roman.as_bytes();
        roman
            .windows(2)
            .map(|window| {
                let (token, next_token) = (roman_value(window[0]), roman_value(window[1]));
                token * if token < next_token { -1 } else { 1 }
            })
            .sum::<i32>()
            + roman_value(*roman.last().unwrap())
    }
}
