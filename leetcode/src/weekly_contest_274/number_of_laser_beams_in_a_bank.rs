crate::solution!();

const LASER_DEVICE: char = '1';

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter()
            .map(|floor| floor.chars().filter(|&ch| ch == LASER_DEVICE).count())
            .filter(|&devices| devices != 0)
            .scan(0, |devices_on_prev_floor, devices_on_floor| {
                let total_lasers = *devices_on_prev_floor * devices_on_floor;
                *devices_on_prev_floor = devices_on_floor;
                Some(total_lasers)
            })
            .sum::<usize>() as i32
    }
}
