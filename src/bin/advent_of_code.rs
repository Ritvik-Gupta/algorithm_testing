use algorithms::advent_of_code::{dumbo_octopus::DumboOctopus, passage_pathing::PassagePathing, *};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run_advent_problem::<PassagePathing>()
}
