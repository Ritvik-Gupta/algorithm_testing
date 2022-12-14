use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone, Copy)]
struct Location(i32, i32);

impl From<(i32, i32)> for Location {
    fn from(loc: (i32, i32)) -> Self {
        Location(loc.0, loc.1)
    }
}

impl<L> std::ops::Add<L> for Location
where
    L: Into<Location>,
{
    type Output = Self;

    fn add(self, other: L) -> Self::Output {
        let other = other.into();
        Location(self.0 + other.0, self.1 + other.1)
    }
}

fn parse_rock_path(input: &str) -> IResult<&str, Vec<Location>> {
    separated_list1(
        tag(" -> "),
        separated_pair(i32, char(','), i32).map(|(x, y)| Location(x, y)),
    )(input)
}

fn cert_range(a: i32, b: i32) -> RangeInclusive<i32> {
    i32::min(a, b)..=i32::max(a, b)
}

pub struct CaveMap {
    grid: BTreeMap<i32, BTreeSet<i32>>,
    floor: Option<i32>,
}

impl CaveMap {
    fn new() -> Self {
        Self {
            grid: BTreeMap::new(),
            floor: None,
        }
    }

    fn add_obstacle(&mut self, loc: impl Into<Location>) {
        let loc = loc.into();
        self.grid
            .entry(loc.0)
            .or_insert_with(BTreeSet::new)
            .insert(loc.1);
    }

    fn has_obstacle(&self, loc: impl Into<Location>) -> bool {
        let loc = loc.into();

        self.grid
            .get(&loc.0)
            .map_or(false, |section| section.contains(&loc.1))
            || self.floor.map_or(false, |floor| floor == loc.1)
    }

    fn obstable_below(&self, loc: impl Into<Location>) -> Option<i32> {
        let loc = loc.into();
        self.grid
            .get(&loc.0)
            .map_or(self.floor.as_ref(), |section| {
                section.range(loc.1 + 1..).chain(self.floor.as_ref()).next()
            })
            .cloned()
    }

    fn build_with_floor(&mut self) {
        self.floor = Some(
            2 + self
                .grid
                .values()
                .map(|section| section.range(..).next_back().unwrap())
                .max()
                .unwrap(),
        )
    }
}

fn simulate_sand_drop(cave_map: &CaveMap, sand_loc: impl Into<Location>) -> Option<Location> {
    let mut sand_loc = sand_loc.into();
    loop {
        let Some (obstable_y) = cave_map.obstable_below(sand_loc) else { return None };
        sand_loc.1 = obstable_y - 1;

        if !cave_map.has_obstacle(sand_loc + (-1, 1)) {
            sand_loc = sand_loc + (-1, 1)
        } else if !cave_map.has_obstacle(sand_loc + (1, 1)) {
            sand_loc = sand_loc + (1, 1);
        } else {
            return Some(sand_loc);
        }
    }
}

pub struct RegolithReservoir;

impl crate::AdventDayProblem for RegolithReservoir {
    type Arg = CaveMap;
    type Ret = i32;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut cave_map = CaveMap::new();
        dataset.for_each(|line| {
            let (_, rock_path) = parse_rock_path(line.as_str()).unwrap();

            rock_path.windows(2).for_each(|path| {
                let (point_a, point_b) = (path[0], path[1]);

                if point_a.0 == point_b.0 {
                    let x = point_a.0;
                    cert_range(point_a.1, point_b.1).for_each(|y| {
                        cave_map.add_obstacle((x, y));
                    });
                } else {
                    let y = point_a.1;
                    cert_range(point_a.0, point_b.0).for_each(|x| {
                        cave_map.add_obstacle((x, y));
                    });
                }
            });
        });
        cave_map
    }

    fn part_1(mut cave_map: Self::Arg) -> Self::Ret {
        let mut units_dropped = 0;
        loop {
            match simulate_sand_drop(&cave_map, (500, 0)) {
                Some(sand_loc) => cave_map.add_obstacle(sand_loc),
                None => return units_dropped,
            }
            units_dropped += 1;
        }
    }

    fn part_2(mut cave_map: Self::Arg) -> Self::Ret {
        cave_map.build_with_floor();

        let mut units_dropped = 0;
        while !cave_map.has_obstacle((500, 0)) {
            let Some(sand_loc) = simulate_sand_drop(&cave_map, (500, 0)) else { unreachable!() };
            cave_map.add_obstacle(sand_loc);
            units_dropped += 1;
        }
        units_dropped
    }
}
