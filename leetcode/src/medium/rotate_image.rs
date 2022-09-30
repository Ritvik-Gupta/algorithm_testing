crate::solution!();

fn swap(ptr1: *mut i32, ptr2: *mut i32) {
    unsafe {
        let temp = *ptr1;
        *ptr1 = *ptr2;
        *ptr2 = temp;
    }
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        (0..n).for_each(|i| (i + 1..n).for_each(|j| swap(&mut matrix[i][j], &mut matrix[j][i])));

        matrix
            .iter_mut()
            .for_each(|row| (0..n / 2).for_each(|j| swap(&mut row[j], &mut row[n - 1 - j])));
    }
}
