use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32 as integer,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::collections::HashMap;
use Color::*;

structstruck::strike! {
    pub struct CubeGame {
        game_id: u32,
        sets: Vec<struct Set {
            cubes: Vec<struct Cube {
                number: u32,
                color:
                    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
                    enum Color {
                        RED, BLUE, GREEN,
                    }
            }>
        }>
    }
}

impl Color {
    fn construct(input: &str) -> IResult<&str, Self> {
        alt((
            tag("red").map(|_| Self::RED),
            tag("blue").map(|_| Self::BLUE),
            tag("green").map(|_| Self::GREEN),
        ))(input)
    }
}

impl Cube {
    fn construct(input: &str) -> IResult<&str, Self> {
        separated_pair(integer, tag(" "), Color::construct)
            .map(|(n, c)| Self {
                number: n,
                color: c,
            })
            .parse(input)
    }
}

impl Set {
    fn construct(input: &str) -> IResult<&str, Self> {
        separated_list1(tag(", "), Cube::construct)
            .map(|c| Self { cubes: c })
            .parse(input)
    }
}

impl CubeGame {
    fn construct(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("Game "),
            separated_pair(
                integer,
                tag(": "),
                separated_list1(tag("; "), Set::construct),
            )
            .map(|(id, s)| CubeGame {
                game_id: id,
                sets: s,
            }),
        )(input)
    }
}

macro_rules! map {
    ($($key: expr => $value: expr),*) => {
        HashMap::from([$(($key, $value)),*])
    };
}

pub struct CubeConundrum;

impl crate::AdventDayProblem for CubeConundrum {
    type Arg = Vec<CubeGame>;
    type Ret = u64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| CubeGame::construct(&line).unwrap().1)
            .collect()
    }

    fn part_1(cube_games: Self::Arg) -> Self::Ret {
        let color_thresholds = map!(RED => 12, GREEN => 13, BLUE => 14);

        cube_games
            .iter()
            .filter_map(|game| {
                game.sets
                    .iter()
                    .all(|set| {
                        set.cubes
                            .iter()
                            .all(|cube| cube.number <= color_thresholds[&cube.color])
                    })
                    .then(|| game.game_id as u64)
            })
            .sum()
    }

    fn part_2(cube_games: Self::Arg) -> Self::Ret {
        cube_games
            .iter()
            .map(|game| {
                let mut min_color_cubes = map!(RED => 0, GREEN => 0, BLUE => 0);
                game.sets.iter().for_each(|set| {
                    set.cubes.iter().for_each(|cube| {
                        min_color_cubes.insert(
                            cube.color,
                            min_color_cubes[&cube.color].max(cube.number as u64),
                        );
                    })
                });

                min_color_cubes
                    .into_values()
                    .reduce(|prod, x| prod * x)
                    .unwrap()
            })
            .sum()
    }
}
