crate::solution!();

type Position = (usize, usize);

fn neighbor_cells(pos: Position) -> [Position; 4] {
    [
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0.wrapping_add(1), pos.1),
        (pos.0, pos.1.wrapping_add(1)),
    ]
}

fn dfs_mark_bordered((x, y): Position, grid: &mut Vec<Vec<i32>>) {
    if x >= grid.len() || y >= grid[0].len() || grid[x][y] != 1 {
        return;
    }

    grid[x][y] = 2;
    neighbor_cells((x, y))
        .iter()
        .for_each(|&pos| dfs_mark_bordered(pos, grid));
}

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m {
            dfs_mark_bordered((i, 0), &mut grid);
            dfs_mark_bordered((i, n - 1), &mut grid);
        }
        for j in 0..n {
            dfs_mark_bordered((0, j), &mut grid);
            dfs_mark_bordered((m - 1, j), &mut grid);
        }

        grid.into_iter()
            .map(|row| row.into_iter().filter(|&cell| cell == 1).count())
            .sum::<usize>() as i32
    }
}
