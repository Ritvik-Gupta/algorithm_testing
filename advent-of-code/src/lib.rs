pub mod binary_diagnostic;
pub mod dive;
pub mod dumbo_octopus;
pub mod extended_polymerization;
pub mod giant_squid;
pub mod lanternfish;
pub mod passage_pathing;
pub mod smoke_basin;
pub mod sonar_sweep;
pub mod syntax_scoring;
pub mod transparent_origami;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub trait AdventDayProblem {
    type Arg;

    fn get_problem_name() -> &'static str;
    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg;

    fn part_1(arg: Self::Arg) -> i128;
    fn part_2(arg: Self::Arg) -> i128;
}

pub fn run_advent_problem<PR: AdventDayProblem>() -> Result<(), Box<dyn Error>> {
    let file = File::open(&format!(
        "./advent-of-code/files/{}.txt",
        PR::get_problem_name()
    ))?;
    println!(
        "{}",
        PR::part_1(PR::construct_arg(
            BufReader::new(&file)
                .lines()
                .map(|line| line.expect("is a valid line")),
        ))
    );

    let file = File::open(&format!(
        "./advent-of-code/files/{}.txt",
        PR::get_problem_name()
    ))?;
    println!(
        "{}",
        PR::part_2(PR::construct_arg(
            BufReader::new(&file)
                .lines()
                .map(|line| line.expect("is a valid line")),
        ))
    );

    Ok(())
}
