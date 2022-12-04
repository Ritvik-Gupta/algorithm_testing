use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn linear_readings(mut all_readings: impl Iterator<Item = usize>) -> Result<usize, Box<dyn Error>> {
    let mut prev_reading = all_readings.next().ok_or("has atleast 1 reading")?;

    Ok(all_readings
        .filter(|&reading| {
            let is_greater = reading > prev_reading;
            prev_reading = reading;
            is_greater
        })
        .count())
}

fn window_of_3_readings(
    mut all_readings: impl Iterator<Item = usize>,
) -> Result<usize, Box<dyn Error>> {
    let mut window = VecDeque::with_capacity(3);
    let mut window_sum = 0;

    for _ in 0..3 {
        let reading = all_readings.next().expect("has atleast 3 readings");
        window.push_back(reading);
        window_sum += reading;
    }

    Ok(all_readings
        .filter(|&reading| {
            let prev_window_sum = window_sum;
            window_sum -= window.pop_front().expect("to not be empty");

            window.push_back(reading);
            window_sum += reading;

            window_sum > prev_window_sum
        })
        .count())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./files/sonar_sweep.txt")?;
    let result = linear_readings(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line").parse().expect("is a number")),
    )?;
    println!("{}", result);

    let file = File::open("./files/sonar_sweep.txt")?;
    let result = window_of_3_readings(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line").parse().expect("is a number")),
    )?;
    println!("{}", result);

    Ok(())
}
