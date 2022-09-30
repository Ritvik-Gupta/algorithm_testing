crate::solution!();

use std::str::Chars;

fn get_cells(mut str: Chars) -> Option<(char, char, char, char)> {
    let (start_col, start_row) = (str.next()?, str.next()?);
    str.next()?;
    let (end_col, end_row) = (str.next()?, str.next()?);

    Some((start_col, start_row, end_col, end_row))
}

impl Solution {
    pub fn cells_in_range(str: String) -> Vec<String> {
        let (start_col, start_row, end_col, end_row) = get_cells(str.chars()).unwrap();

        (start_col..=end_col)
            .flat_map(|col| (start_row..=end_row).map(move |row| format!("{}{}", col, row)))
            .collect()
    }
}
