use algorithms::advent_of_code::*;
use std::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// fn main() -> Result<(), Box<dyn Error>> {
// let file = File::open("./files/sonar_sweep.txt")?;

// let result = sonar_sweep::linear_readings(BufReader::new(&file).lines().map(|line| {
//     line.expect("to be a valid line")
//         .parse()
//         .expect("to be a number")
// }))?;
// println!("{}", result);

// let file = File::open("./files/sonar_sweep.txt")?;

// let result = sonar_sweep::window_of_3_readings(BufReader::new(&file).lines().map(|line| {
//     line.expect("to be a valid line")
//         .parse()
//         .expect("to be a number")
// }))?;
// println!("{}", result);

// Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("./files/dive.txt")?;

//     let result = dive::perform_commands(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("to be a valid line")),
//     )?;
//     println!("{}", result);

//     let file = File::open("./files/dive.txt")?;

//     let result = dive::perform_commands_with_aim(
//         BufReader::new(&file)
//             .lines()
//             .map(|line| line.expect("to be a valid line")),
//     )?;
//     println!("{}", result);

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./files/binary_diagnostic.txt")?;

    let result = binary_diagnostic::power_consumption(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("to be a valid line")),
    )?;
    println!("{}", result);

    Ok(())
}
