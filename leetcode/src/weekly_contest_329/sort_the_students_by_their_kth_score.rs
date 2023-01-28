crate::solution!();

use std::cmp::Reverse;

impl Solution {
    pub fn sort_the_students(mut table: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        table.sort_by_key(|student_marks| Reverse(student_marks[k as usize]));
        table
    }
}
