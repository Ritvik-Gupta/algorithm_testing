use itertools::Itertools;
use nom::{
    character::complete::u32,
    character::{complete::anychar, streaming::char},
    combinator::map,
    error::ErrorKind,
    sequence::separated_pair,
};
use std::collections::HashSet;
use Direction::*;

#[derive(Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location(i32, i32);

impl std::ops::AddAssign for Location {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl From<(i32, i32)> for Location {
    fn from(loc: (i32, i32)) -> Self {
        Self(loc.0, loc.1)
    }
}

struct Bodypart {
    pos: Location,
    visited: HashSet<Location>,
}

impl Bodypart {
    fn new() -> Self {
        Self {
            pos: Location(0, 0),
            visited: HashSet::from([Location(0, 0)]),
        }
    }
}

fn simulate_snake_body<const SNAKE: usize>(moves: &Vec<Direction>) -> [Bodypart; SNAKE] {
    let mut bodyparts = std::array::from_fn(|_| Bodypart::new());
    let to_unit = |num: i32| num / i32::max(1, num.abs());

    moves.iter().for_each(|direction| {
        bodyparts[0].pos += match direction {
            UP => (1, 0),
            DOWN => (-1, 0),
            RIGHT => (0, 1),
            LEFT => (0, -1),
        }
        .into();
        let mut head = bodyparts[0].pos;

        bodyparts[1..].iter_mut().for_each(|body| {
            let body_is_touching_head = (head.0 - 1..=head.0 + 1)
                .cartesian_product(head.1 - 1..=head.1 + 1)
                .any(|near_head| body.pos == near_head.into());

            if !body_is_touching_head {
                body.pos += (to_unit(head.0 - body.pos.0), to_unit(head.1 - body.pos.1)).into();
                body.visited.insert(body.pos);
            }

            head = body.pos;
        });
    });

    bodyparts
}

pub struct RopeBridge;

impl crate::AdventDayProblem for RopeBridge {
    type Arg = Vec<Direction>;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .flat_map(|line| {
                let (_, (direction, steps)) = separated_pair(
                    map(anychar::<_, (_, ErrorKind)>, |dir| match dir {
                        'U' => UP,
                        'D' => DOWN,
                        'R' => RIGHT,
                        'L' => LEFT,
                        _ => unreachable!(),
                    }),
                    char(' '),
                    u32,
                )(line.as_str())
                .unwrap();

                std::iter::repeat(direction).take(steps as usize)
            })
            .collect()
    }

    fn part_1(moves: Self::Arg) -> Self::Ret {
        simulate_snake_body::<2>(&moves)
            .last()
            .unwrap()
            .visited
            .len()
    }

    fn part_2(moves: Self::Arg) -> Self::Ret {
        simulate_snake_body::<10>(&moves)
            .last()
            .unwrap()
            .visited
            .len()
    }
}
