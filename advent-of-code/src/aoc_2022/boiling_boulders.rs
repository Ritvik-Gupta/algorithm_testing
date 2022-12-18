use cached::proc_macro::cached;
use cached::SizedCache;
use derive_more::{Deref, DerefMut};
use nom::{
    character::complete::char as ch, character::complete::i32 as int, combinator::map,
    sequence::separated_pair, IResult,
};
use std::collections::HashSet;

const SPACE: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Pos3([i32; SPACE]);

impl Pos3 {
    const MAX: Pos3 = Pos3([i32::MAX; SPACE]);
    const MIN: Pos3 = Pos3([i32::MIN; SPACE]);

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(int, ch(','), separated_pair(int, ch(','), int)),
            |(x, (y, z))| Self([x, y, z]),
        )(input)
    }

    fn faces<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        FACE_DIRECTIONS.iter().map(|&face_dir| *self + face_dir)
    }
}

impl std::ops::Add for Pos3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Pos3([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}

static FACE_DIRECTIONS: [Pos3; 6] = [
    Pos3([1, 0, 0]),
    Pos3([-1, 0, 0]),
    Pos3([0, 1, 0]),
    Pos3([0, -1, 0]),
    Pos3([0, 0, 1]),
    Pos3([0, 0, -1]),
];

pub struct Obsidian {
    cubes: HashSet<Pos3>,
    dimension_min: Pos3,
    dimension_max: Pos3,
}

#[cached(
    type = "SizedCache<Pos3, bool>",
    create = "{ SizedCache::with_size(100) }",
    convert = "{ pos }"
)]
fn dfs_to_escape(pos: Pos3, obsidian: &Obsidian, seen_cubes: &mut HashSet<Pos3>) -> bool {
    if obsidian.cubes.contains(&pos) || seen_cubes.contains(&pos) {
        return false;
    }
    seen_cubes.insert(pos);

    for i in 0..SPACE {
        if !(obsidian.dimension_min[i] <= pos[i] && pos[i] <= obsidian.dimension_max[i]) {
            return true;
        }
    }

    pos.faces()
        .any(|pos| dfs_to_escape(pos, obsidian, seen_cubes))
}

pub struct BoilingBoulders;

impl crate::AdventDayProblem for BoilingBoulders {
    type Arg = Obsidian;
    type Ret = usize;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let (mut dim_min, mut dim_max) = (Pos3::MAX, Pos3::MIN);

        let cubes = dataset
            .map(|line| {
                let (_, pos) = Pos3::parse(line.as_str()).unwrap();
                for i in 0..SPACE {
                    dim_min[i] = i32::min(dim_min[i], pos[i]);
                    dim_max[i] = i32::max(dim_max[i], pos[i]);
                }
                pos
            })
            .collect();

        Obsidian {
            cubes,
            dimension_min: dim_min,
            dimension_max: dim_max,
        }
    }

    fn part_1(obsidian: Self::Arg) -> Self::Ret {
        obsidian
            .cubes
            .iter()
            .map(|&pos| {
                6 - pos
                    .faces()
                    .filter(|pos| obsidian.cubes.contains(&pos))
                    .count()
            })
            .sum()
    }

    fn part_2(obsidian: Self::Arg) -> Self::Ret {
        tqdm::tqdm(obsidian.cubes.iter())
            .map(|&pos| {
                pos.faces()
                    .map(|pos| dfs_to_escape(pos, &obsidian, &mut HashSet::new()) as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}
