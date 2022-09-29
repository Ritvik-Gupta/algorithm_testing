use std::{collections::VecDeque, convert::TryInto};

use bitset::BitSet;

const GRID_SIZE: usize = 10;
const GRID_AREA: usize = GRID_SIZE * GRID_SIZE;

type Location = (usize, usize);

fn neighboring_octopuses((x, y): Location) -> Vec<Location> {
    let mut neighbors = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            match ((x as isize + i).try_into(), (y as isize + j).try_into()) {
                (Ok(x), Ok(y)) if x < GRID_SIZE && y < GRID_SIZE => neighbors.push((x, y)),
                _ => {}
            }
        }
    }
    neighbors
}

struct FlashingOctopus {
    queue: VecDeque<Location>,
    active_locations: BitSet,
}

impl FlashingOctopus {
    fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(GRID_AREA),
            active_locations: BitSet::with_capacity(GRID_AREA),
        }
    }

    fn add(&mut self, location: Location) {
        self.queue.push_back(location);
        self.active_locations
            .set(location.0 * GRID_SIZE + location.1, true);
    }

    fn reset(&mut self) {
        self.queue.clear();
        self.active_locations.reset();
    }
}

pub struct DumboOctopus;

impl DumboOctopus {
    fn simulate_generation(
        flashing_octopuses: &mut FlashingOctopus,
        energy_levels: &mut Vec<Vec<u8>>,
    ) {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                energy_levels[x][y] += 1;
                if energy_levels[x][y] >= 10 {
                    flashing_octopuses.add((x, y));
                }
            }
        }

        while let Some(flashing_location) = flashing_octopuses.queue.pop_front() {
            neighboring_octopuses(flashing_location)
                .into_iter()
                .for_each(|(x, y)| {
                    if energy_levels[x][y] >= 10 {
                        return;
                    }

                    energy_levels[x][y] += 1;
                    if energy_levels[x][y] >= 10 {
                        flashing_octopuses.add((x, y));
                    }
                });
        }

        energy_levels.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|octopus| **octopus >= 10)
                .for_each(|octopus| *octopus = 0);
        });
    }
}

impl super::AdventDayProblem for DumboOctopus {
    type Arg = Vec<Vec<u8>>;

    fn get_problem_name() -> &'static str {
        file!()
            .split('\\')
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
            .collect()
    }

    fn part_1(mut energy_levels: Self::Arg) -> i128 {
        let mut total_flashes = 0;
        let mut flashing_octopuses = FlashingOctopus::new();

        for _ in 0..100 {
            flashing_octopuses.reset();
            Self::simulate_generation(&mut flashing_octopuses, &mut energy_levels);
            total_flashes += flashing_octopuses.active_locations.count() as i128;
        }

        total_flashes
    }

    fn part_2(mut energy_levels: Self::Arg) -> i128 {
        let mut total_generation_flashes = 0;
        let mut generation_counter = 0;
        let mut flashing_octopuses = FlashingOctopus::new();

        while total_generation_flashes != GRID_AREA {
            flashing_octopuses.reset();
            Self::simulate_generation(&mut flashing_octopuses, &mut energy_levels);
            total_generation_flashes = flashing_octopuses.active_locations.count() as usize;
            generation_counter += 1;
        }

        generation_counter
    }
}
