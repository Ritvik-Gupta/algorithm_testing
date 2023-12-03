use advent_of_code::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run_advent_problem::<Naive, aoc_2023::trebuchet::Trebechut>()
}
