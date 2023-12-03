use std::collections::HashMap;

macro_rules! map {
    ($($key: expr => $value: expr),*) => {
        HashMap::from([$(($key, $value)),*])
    };
}

fn construct_calibration_value(calibration: &String) -> u32 {
    let tens_digit = calibration
        .chars()
        .find_map(|cal| cal.to_digit(10))
        .unwrap_or(0);
    let units_digit = calibration
        .chars()
        .rev()
        .find_map(|cal| cal.to_digit(10))
        .unwrap_or(0);
    tens_digit * 10 + units_digit
}

pub struct Trebechut;

impl crate::AdventDayProblem for Trebechut {
    type Arg = Vec<String>;
    type Ret = u32;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset.filter(|line| line.len() > 0).collect()
    }

    fn part_1(calibrations: Self::Arg) -> Self::Ret {
        calibrations.iter().map(construct_calibration_value).sum()
    }

    fn part_2(calibrations: Self::Arg) -> Self::Ret {
        let digit_table: HashMap<&str, &str> = map! {
            "one" => "o1ne", "two" => "t2wo", "three" => "th3ree", "four" => "fo4ur", "five" => "fi5ve",
            "six" => "s6ix", "seven" => "se7ven", "eight" => "ei8ght", "nine" => "ni9ne", "ten" => "t10en"
        };

        calibrations
            .into_iter()
            .map(|mut calibration| {
                for (rep, val) in digit_table.iter() {
                    calibration = calibration.replace(rep, val);
                }
                construct_calibration_value(&calibration)
            })
            .sum()
    }
}
