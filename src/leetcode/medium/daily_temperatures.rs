pub struct Solution;

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut temperature_stack = Vec::with_capacity(temperatures.len());
        for (pos, temperature) in temperatures.iter_mut().enumerate().rev() {
            let temp = *temperature;
            *temperature = loop {
                match temperature_stack.last() {
                    Some(&(top_temp, _)) if temp >= top_temp => temperature_stack.pop(),
                    Some(&(_, temp_idx)) => break (temp_idx - pos) as i32,
                    None => break 0,
                };
            };
            temperature_stack.push((temp, pos));
        }

        temperatures
    }
}
