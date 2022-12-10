use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32 as integer, combinator::map,
    error::ErrorKind, sequence::preceded,
};
use Command::*;

pub struct CathodeRayTube;

pub enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn cycle_length(&self) -> usize {
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Noop => 0,
            Addx(val) => *val,
        }
    }
}

impl crate::AdventDayProblem for CathodeRayTube {
    type Arg = Vec<i32>;
    type Ret = String;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        let mut register = 1;
        dataset
            .map(|line| {
                let (_, command) = alt((
                    map(tag::<_, _, (_, ErrorKind)>("noop"), |_| Noop),
                    map(preceded(tag("addx "), integer), |val| Addx(val)),
                ))(line.as_str())
                .unwrap();
                command
            })
            .flat_map(|command| {
                let itr = std::iter::repeat(register).take(command.cycle_length());
                register += command.value();
                itr
            })
            .collect()
    }

    fn part_1(register_states: Self::Arg) -> Self::Ret {
        register_states
            .iter()
            .zip(1..)
            .filter(|&(_, cycle)| (cycle - 20) % 40 == 0)
            .map(|(&register, cycle)| register as i128 * cycle)
            .sum::<i128>()
            .to_string()
    }

    fn part_2(register_states: Self::Arg) -> Self::Ret {
        let total_cycles = register_states.len();

        let mut drawing_pixels = register_states
            .iter()
            .zip(0..)
            .map(|(&register, crt_pixel)| {
                (register - 1..=register + 1)
                    .contains(&(crt_pixel % 40))
                    .then_some('#')
                    .unwrap_or('.')
            });

        let drawing_pixels = drawing_pixels.by_ref();

        itertools::Itertools::intersperse(
            (0..total_cycles)
                .step_by(40)
                .map(|_| drawing_pixels.take(40).collect::<String>()),
            "\n".to_string(),
        )
        .collect()
    }
}
