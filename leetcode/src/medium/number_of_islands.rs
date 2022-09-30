crate::solution!();

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::{DOWN, LEFT, RIGHT, UP};

static NEIGHBOUR_DIRS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

struct Vector(usize, usize);

impl std::ops::Add<&Direction> for &Vector {
    type Output = Vector;

    fn add(self, direction: &Direction) -> Vector {
        match direction {
            UP => Vector(self.0 - 1, self.1),
            DOWN => Vector(self.0 + 1, self.1),
            LEFT => Vector(self.0, self.1 - 1),
            RIGHT => Vector(self.0, self.1 + 1),
        }
    }
}

impl Vector {
    fn neighbours(self) -> impl Iterator<Item = Vector> {
        NEIGHBOUR_DIRS
            .iter()
            .map(move |direction| &self + direction)
    }
}

#[derive(PartialEq, Eq)]
enum MapFeature {
    LAND,
    WATER,
}

use MapFeature::{LAND, WATER};

impl Into<MapFeature> for char {
    fn into(self) -> MapFeature {
        match self {
            '0' => WATER,
            '1' => LAND,
            _ => panic!(),
        }
    }
}

impl Into<char> for MapFeature {
    fn into(self) -> char {
        match self {
            WATER => '0',
            LAND => '1',
        }
    }
}

struct GridMap(Vec<Vec<char>>);

impl GridMap {
    fn num_rows(&self) -> usize {
        self.0.len()
    }

    fn num_cols(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, pos: &Vector) -> MapFeature {
        self.0
            .get(pos.0)
            .map(|row| row.get(pos.1).map(|&ch| ch.into()))
            .flatten()
            .unwrap_or(WATER)
    }

    fn explore_island(&mut self, pos: Vector) {
        self.0[pos.0][pos.1] = WATER.into();

        for neighbour_pos in pos.neighbours() {
            if self.get(&neighbour_pos) == LAND {
                self.explore_island(neighbour_pos);
            }
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid_map = GridMap(grid);

        (0..grid_map.num_rows())
            .map(|i| {
                (0..grid_map.num_cols())
                    .filter_map(|j| {
                        if grid_map.0[i][j] == LAND.into() {
                            grid_map.explore_island(Vector(i, j));
                            Some(())
                        } else {
                            None
                        }
                    })
                    .count() as i32
            })
            .sum()
    }
}
