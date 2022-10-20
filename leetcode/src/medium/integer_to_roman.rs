crate::solution!();

static ROMAN_NUMERALS: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::new();
        ROMAN_NUMERALS.iter().for_each(|&(value, symbol)| {
            while num >= value {
                result += symbol;
                num -= value;
            }
        });
        result
    }
}
