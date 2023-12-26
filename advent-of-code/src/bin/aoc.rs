use advent_of_code::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Enabling Logger to use in solutions for debugging
    env_logger::init();

    run_advent_problem::<Naive, aoc_2023::cosmic_expansion::CosmicExpansion>()
}
