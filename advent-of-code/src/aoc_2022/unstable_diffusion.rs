use derive_more::Add;
use std::collections::{hash_map::Entry, HashMap, HashSet};

pub struct UnstableDiffusion;

#[derive(PartialEq, Eq, Hash, Add, Clone, Copy)]
pub struct Location(i32, i32);

impl Location {
    const MIN: Self = Self(i32::MIN, i32::MIN);
    const MAX: Self = Self(i32::MAX, i32::MAX);

    fn radial_neighbor_dirs(&self) -> [Location; 3] {
        match self {
            &Location(x, 0) => [Location(x, -1), Location(x, 0), Location(x, 1)],
            &Location(0, y) => [Location(-1, y), Location(0, y), Location(1, y)],
            _ => panic!("Not a Directional HashSettor"),
        }
    }
}

macro_rules! make_dir {
    ($direction_name: tt => ($x: literal, $y: literal)) => {
        const $direction_name: Location = Location($x, $y);
    };
}

make_dir!(NORTH      => (-1,  0));
make_dir!(NORTH_EAST => (-1,  1));
make_dir!(EAST       => ( 0,  1));
make_dir!(SOUTH_EAST => ( 1,  1));
make_dir!(SOUTH      => ( 1,  0));
make_dir!(SOUTH_WEST => ( 1, -1));
make_dir!(WEST       => ( 0, -1));
make_dir!(NORTH_WEST => (-1, -1));

static ALL_DIRECTIONS: [Location; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

static DIRECTION_ORDER: [Location; 4] = [NORTH, SOUTH, WEST, EAST];

fn simulate_elf_scan_area<const SIMULATION_TIME: usize>(
    elf_locations: &mut HashSet<Location>,
) -> usize {
    let mut order_offset = 0;
    let mut proposed_locations = HashMap::new();

    for simulation_iteration in 1..=SIMULATION_TIME {
        elf_locations.iter().for_each(|&loc| {
            if ALL_DIRECTIONS
                .iter()
                .all(|&delta| !elf_locations.contains(&(loc + delta)))
            {
                return;
            }

            for order in 0..4 {
                let direction = DIRECTION_ORDER[(order + order_offset) % 4];
                if direction
                    .radial_neighbor_dirs()
                    .iter()
                    .map(|&dir| loc + dir)
                    .any(|neighbor_loc| elf_locations.contains(&neighbor_loc))
                {
                    continue;
                };

                match proposed_locations.entry(loc + direction) {
                    Entry::Occupied(entry) => {
                        entry.remove();
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(loc);
                    }
                }
                break;
            }
        });

        if proposed_locations.is_empty() {
            return simulation_iteration;
        }

        proposed_locations.drain().for_each(|(next_loc, loc)| {
            elf_locations.remove(&loc);
            elf_locations.insert(next_loc);
        });

        order_offset = (order_offset + 1) % 4;
    }
    SIMULATION_TIME
}

fn boudning_box_size(locations: &HashSet<Location>) -> usize {
    let (mut min_loc, mut max_loc) = (Location::MAX, Location::MIN);

    locations.iter().for_each(|loc| {
        min_loc = Location(i32::min(min_loc.0, loc.0), i32::min(min_loc.1, loc.1));
        max_loc = Location(i32::max(max_loc.0, loc.0), i32::max(max_loc.1, loc.1));
    });

    (max_loc.0 - min_loc.0 + 1) as usize * (max_loc.1 - min_loc.1 + 1) as usize
}

impl crate::AdventDayProblem for UnstableDiffusion {
    type Arg = HashSet<Location>;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut elf_locations = HashSet::new();
        dataset.zip(0..).for_each(|(line, i)| {
            line.chars().zip(0..).for_each(|(ch, j)| {
                if ch == '#' {
                    elf_locations.insert(Location(i, j));
                }
            });
        });
        elf_locations
    }

    fn part_1(mut elf_locations: Self::Arg) -> Self::Ret {
        simulate_elf_scan_area::<10>(&mut elf_locations);
        boudning_box_size(&elf_locations) - elf_locations.len()
    }

    fn part_2(mut elf_locations: Self::Arg) -> Self::Ret {
        simulate_elf_scan_area::<18_446_744_073_709_551_615>(&mut elf_locations)
    }
}
