pub struct Solution;

const AT_ONES: &[&str] = &[
    "", "One ", "Two ", "Three ", "Four ", "Five ", "Six ", "Seven ", "Eight ", "Nine ",
];

const AT_TENS: &[&str] = &[
    "", "", "Twenty ", "Thirty ", "Forty ", "Fifty ", "Sixty ", "Seventy ", "Eighty ", "Ninety ",
];

const WITH_1_AT_TENS: &[&str] = &[
    "Ten ",
    "Eleven ",
    "Twelve ",
    "Thirteen ",
    "Fourteen ",
    "Fifteen ",
    "Sixteen ",
    "Seventeen ",
    "Eighteen ",
    "Nineteen ",
];

const PARTITION_LEVELS: &[&str] = &["", "Thousand ", "Million ", "Billion "];

const NUMERIC_PARTITION: i32 = 1000;

impl Solution {
    fn subnum_to_english(mut num: i32) -> String {
        let num: Vec<_> = std::iter::repeat_with(|| {
            let rem = num % 10;
            num /= 10;
            rem as usize
        })
        .take(3)
        .collect();

        let (ones_digit, tens_digit, hundreds_digit) = (num[0], num[1], num[2]);
        let mut result = "".to_string();

        if hundreds_digit != 0 {
            result = result + AT_ONES[hundreds_digit] + "Hundred ";
        }

        return if tens_digit == 1 {
            result + WITH_1_AT_TENS[ones_digit]
        } else {
            result + AT_TENS[tens_digit] + AT_ONES[ones_digit]
        };
    }

    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let mut result = "".to_string();
        for &partition_level in PARTITION_LEVELS {
            if num == 0 {
                break;
            }

            let three_digits = num % NUMERIC_PARTITION;
            num /= NUMERIC_PARTITION;
            let subnum = Solution::subnum_to_english(three_digits);
            if subnum != "" {
                result = subnum + partition_level + &result;
            }
        }
        result.pop();
        result
    }
}
