pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        use std::collections::BTreeMap;

        let mut roman_numerals: BTreeMap<i32, &str> = BTreeMap::new();
        roman_numerals.insert(1000, "M");
        roman_numerals.insert(900, "CM");
        roman_numerals.insert(500, "D");
        roman_numerals.insert(400, "CD");
        roman_numerals.insert(100, "C");
        roman_numerals.insert(90, "XC");
        roman_numerals.insert(50, "L");
        roman_numerals.insert(40, "XL");
        roman_numerals.insert(10, "X");
        roman_numerals.insert(9, "IX");
        roman_numerals.insert(5, "V");
        roman_numerals.insert(4, "IV");
        roman_numerals.insert(1, "I");

        let mut result = "".to_string();
        roman_numerals.iter().rev().for_each(|(&value, &symbol)| {
            while num >= value {
                result += symbol;
                num -= value;
            }
        });
        result
    }
}
