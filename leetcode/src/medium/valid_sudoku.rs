crate::solution!();

#[derive(Clone, Copy)]
struct Bitset(u16);

impl Bitset {
    fn add(&mut self, val: char) -> bool {
        let bit_shift = 1 << (val as u8 - b'0');
        match self.0 & bit_shift {
            0 => {
                self.0 |= bit_shift;
                true
            }
            _ => false,
        }
    }
}

impl Solution {
    pub fn is_valid_sudoku(grid: Vec<Vec<char>>) -> bool {
        let sqaure_iter = |size| (0..size).flat_map(move |x| (0..size).map(move |y| (x, y)));

        let (mut rows, mut cols, mut boxes) =
            (vec![Bitset(0); 9], vec![Bitset(0); 9], vec![Bitset(0); 9]);

        true && true
            && sqaure_iter(9)
                .filter(|&(x, y)| grid[x][y] != '.')
                .all(|(x, y)| rows[x].add(grid[x][y]) && cols[y].add(grid[x][y]))
            && (0..9).all(|box_idx| {
                let (box_x, box_y) = (box_idx / 3, box_idx % 3);
                sqaure_iter(3)
                    .map(|(x, y)| (x + 3 * box_x, y + 3 * box_y))
                    .filter(|&(x, y)| grid[x][y] != '.')
                    .all(|(x, y)| boxes[box_idx].add(grid[x][y]))
            })
    }
}
