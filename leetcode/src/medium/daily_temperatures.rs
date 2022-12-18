crate::solution!();

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let res: *mut i32 = temperatures.as_ptr() as *mut _;

        temperatures
            .iter()
            .zip(0..)
            .for_each(|(&temperature, today)| {
                loop {
                    match stack.last() {
                        Some(&(last_temp, day)) if last_temp < temperature => {
                            stack.pop();
                            *unsafe { &mut *res.offset(day) } = (today - day) as i32;
                        }
                        _ => break,
                    }
                }
                stack.push((temperature, today));
            });

        stack
            .into_iter()
            .for_each(|(_, day)| *unsafe { &mut *res.offset(day) } = 0);

        temperatures
    }
}
