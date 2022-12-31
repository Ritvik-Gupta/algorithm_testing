crate::solution!();

fn dfs(
    grid: &mut Vec<Vec<i32>>,
    (x, y): (usize, usize),
    cells_counted: usize,
    empty_cells: usize,
    num_paths: &mut i32,
) {
    if x >= grid.len() || y >= grid[0].len() || grid[x][y] == -1 {
        return;
    }

    if grid[x][y] == 2 {
        if cells_counted == empty_cells {
            *num_paths += 1;
        }
        return;
    }

    grid[x][y] = -1;

    dfs(grid, (x + 1, y), cells_counted + 1, empty_cells, num_paths);
    dfs(grid, (x - 1, y), cells_counted + 1, empty_cells, num_paths);
    dfs(grid, (x, y + 1), cells_counted + 1, empty_cells, num_paths);
    dfs(grid, (x, y - 1), cells_counted + 1, empty_cells, num_paths);

    grid[x][y] = 0;
}

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut empty_cells = 1;
        let mut start = (0, 0);

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => empty_cells += 1,
                    1 => start = (i, j),
                    _ => {}
                }
            }
        }

        let mut num_paths = 0;
        dfs(&mut grid, start, 0, empty_cells, &mut num_paths);
        return num_paths;
    }
}
