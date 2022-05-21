crate::leetcode::solution!();

fn time(time_str: String) -> Option<u32> {
    let mut chars = time_str.chars();
    let hrs = 10 * chars.next()?.to_digit(10)? + chars.next()?.to_digit(10)?;
    chars.next()?;
    let mns = 10 * chars.next()?.to_digit(10)? + chars.next()?.to_digit(10)?;
    Some(hrs * 60 + mns)
}

fn fetch_time(time_str: String) -> u32 {
    time(time_str).unwrap()
}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        [60, 15, 5, 1]
            .iter()
            .scan(
                fetch_time(correct) - fetch_time(current),
                |delta, &delta_increment| {
                    let num_steps = *delta / delta_increment;
                    *delta %= delta_increment;
                    Some(num_steps)
                },
            )
            .sum::<u32>() as i32
    }
}
