use crate::utils::Vector;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

fn parse_rock_path(input: &str) -> IResult<&str, Vec<Vector<i32>>> {
    separated_list1(
        tag(" -> "),
        separated_pair(i32, char(','), i32).map(|(x, y)| Vector(x, y)),
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
    const SAND_DROP_FROM: Vector<i32> = Vector(500, 0);

    fn new() -> Self {
        Self {
            grid: BTreeMap::new(),
            floor: None,
        }
    }

    fn add_obstacle(&mut self, loc: impl Into<Vector<i32>>) {
        let loc = loc.into();
        self.grid
            .entry(loc.0)
            .or_insert_with(BTreeSet::new)
            .insert(loc.1);
    }

    fn has_obstacle(&self, loc: impl Into<Vector<i32>>) -> bool {
        let loc = loc.into();

        self.grid
            .get(&loc.0)
            .map_or(false, |section| section.contains(&loc.1))
            || self.floor.map_or(false, |floor| floor == loc.1)
    }

    fn obstable_below(&self, loc: impl Into<Vector<i32>>) -> Option<i32> {
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

    fn simulate_sand_drop(&self, sand_loc: impl Into<Vector<i32>>) -> Option<Vector<i32>> {
        let mut sand_loc = sand_loc.into();
        loop {
            let Some (obstable_y) = self.obstable_below(sand_loc) else { return None };
            sand_loc.1 = obstable_y - 1;

            if !self.has_obstacle(sand_loc + Vector(-1, 1)) {
                sand_loc = sand_loc + Vector(-1, 1)
            } else if !self.has_obstacle(sand_loc + Vector(1, 1)) {
                sand_loc = sand_loc + Vector(1, 1);
            } else {
                return Some(sand_loc);
            }
        }
    }

    fn run_simulation(&mut self) -> i32 {
        let mut units_dropped = 0;
        while !self.has_obstacle(Self::SAND_DROP_FROM) {
            match self.simulate_sand_drop(Self::SAND_DROP_FROM) {
                Some(sand_loc) => self.add_obstacle(sand_loc),
                None => break,
            }
            units_dropped += 1;
        }
        units_dropped
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

            rock_path
                .iter()
                .tuple_windows()
                .for_each(|(point_a, point_b)| {
                    let x_rng = cert_range(point_a.0, point_b.0);
                    let y_rng = cert_range(point_a.1, point_b.1);

                    x_rng
                        .cartesian_product(y_rng)
                        .for_each(|loc| cave_map.add_obstacle(loc));
                });
        });
        cave_map
    }

    fn part_1(mut cave_map: Self::Arg) -> Self::Ret {
        cave_map.run_simulation()
    }

    fn part_2(mut cave_map: Self::Arg) -> Self::Ret {
        cave_map.build_with_floor();
        cave_map.run_simulation()
    }
}
