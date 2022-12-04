crate::solution!();

#[derive(Clone, PartialEq)]
enum Cell {
    Obstructed,
    Guarded,
    Unguarded,
}

fn guard_watches<'a>(cells: impl Iterator<Item = &'a mut Cell> + 'a, unguarded_cells: &mut i32) {
    for cell in cells {
        match *cell {
            Cell::Obstructed => break,
            Cell::Unguarded => *unguarded_cells -= 1,
            _ => {}
        }
        *cell = Cell::Guarded;
    }
}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut unguarded_cells = m * n;
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![Cell::Unguarded; n]; m];

        guards.iter().chain(walls.iter()).for_each(|loc| {
            grid[loc[0] as usize][loc[1] as usize] = Cell::Obstructed;
            unguarded_cells -= 1;
        });

        guards
            .iter()
            .map(|guard_loc| (guard_loc[0] as usize, guard_loc[1] as usize))
            .for_each(|(x, y)| {
                guard_watches(
                    grid[0..x].iter_mut().rev().map(|row| &mut row[y]),
                    &mut unguarded_cells,
                );
                guard_watches(
                    grid[x + 1..m].iter_mut().map(|row| &mut row[y]),
                    &mut unguarded_cells,
                );
                guard_watches(grid[x][0..y].iter_mut().rev(), &mut unguarded_cells);
                guard_watches(grid[x][y + 1..n].iter_mut(), &mut unguarded_cells);
            });

        unguarded_cells
    }
}
