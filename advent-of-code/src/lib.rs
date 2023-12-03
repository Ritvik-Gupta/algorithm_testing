pub mod aoc_2021;
pub mod aoc_2022;
pub mod aoc_2023;
pub mod utils;

use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

macro_rules! problem_name {
    () => {
        file!()
            .split('find_os_specific_symbol: {
                #[cfg(target_os = "windows")]
                break 'find_os_specific_symbol '\\';

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                break 'find_os_specific_symbol '/';

                #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
                panic!("Invalid OS identified");
            })
            .last()
            .expect("has a file path")
            .split('.')
            .next()
            .expect("has a file name")
    };
}
pub(crate) use problem_name;

pub trait OptimizationFlag {}

pub struct Naive;

impl OptimizationFlag for Naive {}

pub trait AdventDayProblem<OF: OptimizationFlag = Naive> {
    type Arg;
    type Ret: Display;

    fn get_problem_name() -> &'static str;
    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg;

    fn part_1(arg: Self::Arg) -> Self::Ret;
    fn part_2(arg: Self::Arg) -> Self::Ret;
}

pub fn run_advent_problem<OF: OptimizationFlag, PR: AdventDayProblem<OF>>(
) -> Result<(), Box<dyn Error>> {
    let file_path = format!("./advent-of-code/files/{}.txt", PR::get_problem_name());

    let file = File::open(&file_path).expect(&format!("File path '{}' not present", file_path));
    println!(
        "{}",
        PR::part_1(PR::construct_arg(
            BufReader::new(&file)
                .lines()
                .map(|line| line.expect("is a valid line")),
        ))
    );

    let file = File::open(&file_path).expect(&format!("File path '{}' not present", file_path));
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
