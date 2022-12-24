use crate::utils::Vector;
use bit_set::BitSet;
use keyed_priority_queue::{Entry::*, KeyedPriorityQueue};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::cmp::Reverse;

pub struct HillClimbingAlgorithm;

impl Vector<usize> {
    fn area(&self) -> usize {
        self.0 * self.1
    }

    fn is_within(&self, other: &Self) -> bool {
        self.0 < other.0 && self.1 < other.1
    }

    fn project_in(&self, other: &Self) -> usize {
        self.0 * other.1 + self.1
    }

    fn neighbors(&self) -> impl Iterator<Item = Vector<usize>> {
        [
            Vector(self.0.wrapping_add(1), self.1),
            Vector(self.0.wrapping_sub(1), self.1),
            Vector(self.0, self.1.wrapping_add(1)),
            Vector(self.0, self.1.wrapping_sub(1)),
        ]
        .into_iter()
    }
}

pub struct WorldMap {
    grid: Vec<Vec<i8>>,
    start: Vector<usize>,
    end: Vector<usize>,
}

impl WorldMap {
    fn size(&self) -> Vector<usize> {
        Vector(self.grid.len(), self.grid[0].len())
    }

    fn find_all_grounds<'it>(&'it self) -> impl ParallelIterator<Item = Vector<usize>> + 'it {
        self.grid.par_iter().enumerate().flat_map(|(x, row)| {
            row.par_iter()
                .enumerate()
                .filter(|(_, &cell)| cell == 0)
                .map(move |(y, _)| Vector(x, y))
        })
    }

    fn dijkstras_algo(&self, start: Vector<usize>) -> usize {
        let grid_bounds = self.size();

        let mut cell_queue = KeyedPriorityQueue::new();
        let mut visited_cells = BitSet::with_capacity(grid_bounds.area());
        cell_queue.push(start, Reverse(0));

        while let Some((cell, Reverse(path_length))) = cell_queue.pop() {
            if cell == self.end {
                return path_length;
            }

            for ngb_cell in cell
                .neighbors()
                .filter(|cell| cell.is_within(&grid_bounds))
                .filter(|cell| !visited_cells.contains(cell.project_in(&grid_bounds)))
                .filter(|&ngb_cell| self[ngb_cell] - self[cell] <= 1)
            {
                match cell_queue.entry(ngb_cell) {
                    Occupied(entry) => {
                        if entry.get_priority().0 > path_length + 1 {
                            entry.set_priority(Reverse(path_length + 1));
                        }
                    }
                    Vacant(entry) => entry.set_priority(Reverse(path_length + 1)),
                }
            }
            visited_cells.insert(cell.project_in(&grid_bounds));
        }
        usize::MAX
    }
}

impl std::ops::Index<Vector<usize>> for WorldMap {
    type Output = i8;

    fn index(&self, loc: Vector<usize>) -> &Self::Output {
        &self.grid[loc.0][loc.1]
    }
}

impl crate::AdventDayProblem for HillClimbingAlgorithm {
    type Arg = WorldMap;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let (mut start, mut end) = (Vector::default(), Vector::default());

        let grid = dataset
            .enumerate()
            .map(|(x, line)| {
                line.char_indices()
                    .map(|(y, token)| {
                        let token = match token {
                            'S' => {
                                start = Vector(x, y);
                                'a'
                            }
                            'E' => {
                                end = Vector(x, y);
                                'z'
                            }
                            _ => token,
                        };
                        (token as u8 - b'a') as i8
                    })
                    .collect()
            })
            .collect();

        WorldMap { grid, start, end }
    }

    fn part_1(map: Self::Arg) -> Self::Ret {
        map.dijkstras_algo(map.start)
    }

    fn part_2(map: Self::Arg) -> Self::Ret {
        map.find_all_grounds()
            .map(|start| map.dijkstras_algo(start))
            .min()
            .unwrap()
    }
}
