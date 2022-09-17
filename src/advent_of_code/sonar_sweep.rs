use std::{collections::VecDeque, error::Error};

pub fn linear_readings(
    mut all_readings: impl Iterator<Item = usize>,
) -> Result<usize, Box<dyn Error>> {
    let mut prev_reading = all_readings.next().ok_or("has atleast 1 reading")?;

    Ok(all_readings
        .filter(|&reading| {
            let is_greater = reading > prev_reading;
            prev_reading = reading;
            is_greater
        })
        .count())
}

pub fn window_of_3_readings(
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
