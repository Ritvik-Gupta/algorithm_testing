crate::leetcode::solution!();

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.as_bytes();
        let mut result_integer = -1;

        (0..num.len() - 2)
            .filter(|&idx| num[idx] == num[idx + 1] && num[idx] == num[idx + 2])
            .for_each(|idx| result_integer = i32::max((num[idx] - b'0') as i32, result_integer));

        if result_integer == -1 {
            return "".to_owned();
        }

        format!("{}", result_integer).repeat(3)
    }
}
