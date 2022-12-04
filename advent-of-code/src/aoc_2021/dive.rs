use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

enum SubmarineDirection {
    FORWARD,
    DOWN,
    UP,
}

use SubmarineDirection::*;

struct SubmarineCommand {
    direction: SubmarineDirection,
    units: i64,
}

impl From<String> for SubmarineCommand {
    fn from(command: String) -> Self {
        let mut split = command.split(' ');
        let (dir, units) = (
            split.next().expect("to have direction info"),
            split.next().expect("to have unit info"),
        );

        Self {
            direction: match dir {
                "forward" => FORWARD,
                "down" => DOWN,
                "up" => UP,
                _ => unreachable!(),
            },
            units: units.parse().expect("to be a number"),
        }
    }
}

fn perform_commands<T>(commands: impl Iterator<Item = T>) -> Result<i64, Box<dyn Error>>
where
    SubmarineCommand: From<T>,
{
    let (horizontal, vertical) =
        commands
            .map(SubmarineCommand::from)
            .fold((0, 0), |(horizontal, vertical), command| {
                match command.direction {
                    FORWARD => (horizontal + command.units, vertical),
                    DOWN => (horizontal, vertical + command.units),
                    UP => (horizontal, vertical - command.units),
                }
            });
    Ok(horizontal * vertical)
}

fn perform_commands_with_aim<T>(commands: impl Iterator<Item = T>) -> Result<i64, Box<dyn Error>>
where
    SubmarineCommand: From<T>,
{
    let (mut horizontal, mut vertical, mut aim) = (0, 0, 0);
    commands
        .map(SubmarineCommand::from)
        .for_each(|command| match command.direction {
            FORWARD => {
                horizontal += command.units;
                vertical += aim * command.units;
            }
            DOWN => aim += command.units,
            UP => aim -= command.units,
        });
    Ok(horizontal * vertical)
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./files/dive.txt")?;

    let result = perform_commands(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line")),
    )?;
    println!("{}", result);

    let file = File::open("./files/dive.txt")?;

    let result = perform_commands_with_aim(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line")),
    )?;
    println!("{}", result);

    Ok(())
}
