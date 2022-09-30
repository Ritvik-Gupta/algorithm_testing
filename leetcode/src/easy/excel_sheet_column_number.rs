crate::solution!();

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .fold(0, |col_num, ch| col_num * 26 + (ch as u8 - b'A' + 1) as i32)
    }
}
