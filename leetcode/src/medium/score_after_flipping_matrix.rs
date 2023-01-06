crate::solution!();

fn flip_bits<'a>(bits: impl Iterator<Item = &'a mut i32> + 'a) {
    bits.for_each(|bit| *bit ^= 1);
}

fn binary_to_decimal(binary: &Vec<i32>) -> i32 {
    binary.iter().fold(0, |dec, bit| dec << 1 | bit)
}

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m {
            if grid[i][0] == 0 {
                flip_bits(grid[i].iter_mut());
            }
        }

        for j in 0..n {
            if grid.iter().map(|r| r[j]).filter(|&b| b == 1).count() <= m / 2 {
                flip_bits(grid.iter_mut().map(|row| &mut row[j]));
            }
        }

        grid.iter().map(binary_to_decimal).sum()
    }
}
