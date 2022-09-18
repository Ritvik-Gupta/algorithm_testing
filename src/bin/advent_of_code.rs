use algorithms::advent_of_code::*;
use std::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

//? Day 1

// fn main() -> Result<(), Box<dyn Error>> {
// let file = File::open("./files/sonar_sweep.txt")?;

// let result = sonar_sweep::linear_readings(BufReader::new(&file).lines().map(|line| {
//     line.expect("is a valid line")
//         .parse()
//         .expect("is a number")
// }))?;
// println!("{}", result);

// let file = File::open("./files/sonar_sweep.txt")?;

// let result = sonar_sweep::window_of_3_readings(BufReader::new(&file).lines().map(|line| {
//     line.expect("is a valid line")
//         .parse()
//         .expect("is a number")
// }))?;
// println!("{}", result);

// Ok(())
// }

//? Day 2

// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("./files/dive.txt")?;

//     let result = dive::perform_commands(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("is a valid line")),
//     )?;
//     println!("{}", result);

//     let file = File::open("./files/dive.txt")?;

//     let result = dive::perform_commands_with_aim(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("is a valid line")),
//     )?;
//     println!("{}", result);

//     Ok(())
// }

//? Day 3

// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("./files/binary_diagnostic.txt")?;

//     let result = binary_diagnostic::power_consumption(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("is a valid line")),
//     )?;
//     println!("{}", result);

//     let file = File::open("./files/binary_diagnostic.txt")?;

//     let mut dataset = BufReader::new(&file)
//         .lines()
//         .map(|line| line.expect("is a valid line"))
//         .peekable();
//     let total_bits = dataset.peek().expect("has atleast one diagnostic").len();

//     let dataset = dataset
//         .map(|line| usize::from_str_radix(&line, 2).expect("is in expected binary number format"))
//         .collect();

//     let result = binary_diagnostic::life_support_rating(dataset, total_bits)?;
//     println!("{}", result);

//     Ok(())
// }

//? Day 4

// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("./files/giant_squid.txt")?;

//     let mut bingo = giant_squid::Bingo::parse(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("is a valid line")),
//     );
//     println!("{}", bingo.first_winning_board());

//     let file = File::open("./files/giant_squid.txt")?;

//     let mut bingo = giant_squid::Bingo::parse(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("is a valid line")),
//     );
//     println!("{}", bingo.last_winning_board());

//     Ok(())
// }

//? Day 5

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./files/hydrothermal_venture.txt")?;

    let mut result = hydrothermal_venture::solve(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line")),
    );
    println!("{}", result);

    Ok(())
}
