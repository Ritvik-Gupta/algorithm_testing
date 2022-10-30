crate::solution!();

use std::collections::VecDeque;

struct Location(usize, usize);

impl Location {
    fn neighboring_cells(&self) -> impl Iterator<Item = Self> {
        vec![
            Self(self.0.wrapping_sub(1), self.1),
            Self(self.0, self.1.wrapping_sub(1)),
            Self(self.0.wrapping_add(1), self.1),
            Self(self.0, self.1.wrapping_add(1)),
        ]
        .into_iter()
    }

    fn is_within(&self, other: &Self) -> bool {
        self.0 < other.0 && self.1 < other.1
    }
}

struct WalkerLocation {
    location: Location,
    obstacles_eliminated: i32,
    distance_covered: i32,
}

impl Solution {
    pub fn shortest_path(mut grid: Vec<Vec<i32>>, max_obstacles_elimination: i32) -> i32 {
        let grid_bounds = Location(grid.len(), grid[0].len());

        grid.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|cell| *cell += (max_obstacles_elimination + 1) * 10);
        });

        let mut queue = VecDeque::new();
        queue.push_back(WalkerLocation {
            location: Location(0, 0),
            obstacles_eliminated: 0,
            distance_covered: 0,
        });

        while let Some(WalkerLocation {
            location,
            mut obstacles_eliminated,
            distance_covered,
        }) = queue.pop_front()
        {
            if location.0 == grid_bounds.0 - 1 && location.1 == grid_bounds.1 - 1 {
                return distance_covered;
            }

            let cell = &mut grid[location.0][location.1];

            if *cell % 10 == 1 {
                if obstacles_eliminated < max_obstacles_elimination {
                    obstacles_eliminated += 1;
                } else {
                    continue;
                }
            }

            if *cell / 10 <= obstacles_eliminated {
                continue;
            }
            *cell = obstacles_eliminated * 10 + *cell % 10;

            location
                .neighboring_cells()
                .filter(|loc| loc.is_within(&grid_bounds))
                .for_each(|location| {
                    queue.push_back(WalkerLocation {
                        location,
                        obstacles_eliminated,
                        distance_covered: distance_covered + 1,
                    });
                });
        }

        -1
    }
}
