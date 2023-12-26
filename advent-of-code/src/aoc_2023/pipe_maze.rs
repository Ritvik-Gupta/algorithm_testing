use derive_more::{Deref, DerefMut};
use std::collections::{HashSet, VecDeque};

use crate::utils::Vector;

static LEFT: Vector<isize> = Vector(0, -1);
static RIGHT: Vector<isize> = Vector(0, 1);
static UP: Vector<isize> = Vector(-1, 0);
static DOWN: Vector<isize> = Vector(1, 0);

fn flip_vector(vec: Vector<isize>) -> Vector<isize> {
    Vector(vec.0 * -1, vec.1 * -1)
}

macro_rules! set {
    ($($elm: expr),*) => {
        HashSet::from([$($elm),*])
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pipe(u8);

impl Pipe {
    fn direction_vectors(&self) -> HashSet<Vector<isize>> {
        match self.0 {
            b'-' => set!(LEFT, RIGHT),
            b'|' => set!(UP, DOWN),
            b'J' => set!(LEFT, UP),
            b'7' => set!(LEFT, DOWN),
            b'L' => set!(RIGHT, UP),
            b'F' => set!(RIGHT, DOWN),
            b'S' => set!(LEFT, RIGHT, UP, DOWN),
            _ => unreachable!(),
        }
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe(b'.')
    }
}

#[derive(Deref, DerefMut)]
pub struct Maze<T>(Vec<Vec<T>>);

impl<T> Maze<T> {
    fn at(&self, pos: Vector<isize>) -> T
    where
        T: Default + Clone,
    {
        self.get(pos.0 as usize)
            .and_then(|row| row.get(pos.1 as usize).cloned())
            .unwrap_or_default()
    }

    fn set(&mut self, pos: Vector<isize>, new_value: T) {
        if let Some(elm) = self
            .get_mut(pos.0 as usize)
            .and_then(|row| row.get_mut(pos.1 as usize))
        {
            *elm = new_value;
        }
    }
}

pub struct PipeMaze;

impl crate::AdventDayProblem for PipeMaze {
    type Arg = Maze<Pipe>;

    type Ret = isize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        Maze(
            dataset
                .map(|line| line.chars().map(|c| Pipe(c as u8)).collect())
                .collect(),
        )
    }

    fn part_1(maze: Self::Arg) -> Self::Ret {
        let mut power_maze = Maze(vec![vec![-1; maze[0].len()]; maze.len()]);
        let mut positions_queue = VecDeque::new();

        let start_loc = maze
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .position(|&c| c == Pipe(b'S'))
                    .and_then(|j| Some((i as isize, j as isize)))
            })
            .unwrap();
        let start_loc = Vector::from(start_loc);

        power_maze.set(start_loc, 0);
        positions_queue.push_back(start_loc);

        while let Some(current_pos) = positions_queue.pop_front() {
            log::info!(
                "Current Pos : ({}, {}) => {}\n",
                current_pos.0,
                current_pos.1,
                maze.at(current_pos).0 as char
            );

            for dir in maze.at(current_pos).direction_vectors() {
                let pos = current_pos + dir;
                let power = power_maze.at(pos);

                log::info!(
                    "Checking Pos : ({}, {}) => {}",
                    pos.0,
                    pos.1,
                    maze.at(pos).0 as char
                );

                if maze.at(pos) == Pipe::default()
                    || !maze.at(pos).direction_vectors().contains(&flip_vector(dir))
                {
                    continue;
                }

                if power == -1 {
                    log::info!(
                        "Marking Pos : ({}, {}) => {}\n",
                        pos.0,
                        pos.1,
                        maze.at(pos).0 as char
                    );

                    positions_queue.push_back(pos);
                    power_maze.set(pos, power_maze.at(current_pos) + 1);
                } else if power == power_maze.at(current_pos) + 1 {
                    return power;
                }
            }
        }
        unreachable!()
    }

    fn part_2(maze: Self::Arg) -> Self::Ret {
        todo!()
    }
}
